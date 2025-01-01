use anyhow::Result;
use clap::Parser;
use inquire::Text;

use crate::{
    cfg::JkConfig,
    clients::gh::{GithubClient, PrData},
    tools::{git::GitClient, inputs::pr_comment},
};

#[derive(Debug, Parser)]
pub struct NewArgs {}

#[derive(Debug, Parser)]
pub enum PrCommand {
    /// Create a new pull request
    New(NewArgs),
}

impl PrCommand {
    pub async fn run_cmd(self, cfg: JkConfig) -> Result<String> {
        match self {
            Self::New(ref args) => self.create_pull_request(cfg).await,
        }
    }

    /// Commit all changes and create a pull request on Github
    pub async fn create_pull_request(self, cfg: JkConfig) -> Result<String> {
        let commit_template = cfg.git_commit_template.clone();
        let git_client = GitClient::new(cfg)?;
        let gh_client = GithubClient::default();
        let (org, repo) = git_client.get_org_repo()?;
        let branch_name = Text::new("Branch name:").prompt()?;
        let commit_msg = pr_comment(commit_template)?;
        git_client.add_all_commit(branch_name.clone(), commit_msg.clone())?;
        git_client.push("create-pr".to_string())?;
        let pr_data = PrData::new(branch_name.clone(), commit_msg, "main".to_string());
        gh_client.create_pull_request(org, repo, pr_data).await?;

        Ok("Ok".to_string())
    }
}
