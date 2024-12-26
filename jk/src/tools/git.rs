use std::{env, path::Path};

use anyhow::{anyhow, Error, Result};
use git2::{Cred, IndexAddOption, Oid, PushOptions, RemoteCallbacks, Repository};
use gpgme::{Context, Protocol, SignMode};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::cfg::JkConfig;

pub struct GitClient {
    user_cfg: JkConfig,
    pub repo: Repository,
}

impl GitClient {
    /// Create new Git client
    pub fn new(user_cfg: JkConfig) -> Result<Self> {
        let repo = Repository::discover(env::current_dir()?)?;
        Ok(GitClient { user_cfg, repo })
    }

    /// Add all to a GPG-signed local commit
    pub fn add_all_commit(&self, branch_name: String, msg: String) -> Result<Oid> {
        // add all files to index
        let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
        let mut index = self.repo.index().unwrap();
        index.add_all(["."], IndexAddOption::DEFAULT, None)?;
        index.write().unwrap();
        let tree = self.repo.find_tree(index.write_tree()?)?;
        let sig = self.repo.signature()?;

        // generate signed commit
        let mut parents = vec![];
        if let Ok(head) = self.repo.head() {
            parents.push(head.peel_to_commit()?);
        }
        let parents = parents.iter().collect::<Vec<_>>();
        let buf = self
            .repo
            .commit_create_buffer(&sig, &sig, &msg, &tree, &parents)?;
        let contents = std::str::from_utf8(&buf).unwrap().to_string();
        let mut outbuf = Vec::new();
        ctx.set_armor(true);
        ctx.sign(SignMode::Detached, buf.as_str().unwrap(), &mut outbuf)?;
        let out = std::str::from_utf8(&outbuf).unwrap();

        // commit and add to repository branch
        let commit_oid = self.repo.commit_signed(&contents, &out, None)?;
        let commit = self.repo.find_commit(commit_oid)?;
        self.repo.branch(&branch_name, &commit, false)?;
        Ok(commit_oid)
    }

    /// Push to a remote git branch
    pub fn push(&self, remote: String) -> Result<()> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                Path::new(&self.user_cfg.ssh_key_fp),
                None,
            )
        });
        let mut origin = self.repo.find_remote("origin").unwrap();
        let mut po = PushOptions::new();
        po.remote_callbacks(callbacks);
        origin
            .push(&[format!("refs/heads/{}", remote)], Some(&mut po))
            .map_err(Error::from)
    }

    fn find_remote_url(&self) -> Result<String> {
        let remote = self.repo.find_remote("origin")?;
        if let Some(n) = remote.url() {
            Ok(n.to_string())
        } else {
            Err(anyhow!("No remote name, wut?!?!"))
        }
    }

    pub fn get_org_repo(&self) -> Result<(String, String)> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"git@github\.com:(?<org>.*)\/(?<repo>.*)\.git").expect("regex should work")
        });
        let remote_url = self.find_remote_url()?;
        let caps = RE.captures(&remote_url).unwrap();
        Ok((caps["org"].to_string(), caps["repo"].to_string()))
    }
}
