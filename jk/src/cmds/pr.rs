use anyhow::Result;
use clap::Parser;
use inquire::Text;

use crate::{
    cfg::JkConfig,
    clients::gh::{GithubClient, PrData},
    tools::git::GitClient,
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

    pub async fn create_pull_request(self, cfg: JkConfig) -> Result<String> {
        let git_client = GitClient::new(cfg)?;
        let gh_client = GithubClient::default();
        let (org, repo) = git_client.get_org_repo()?;
        let branch_name = Text::new("Branch name:").prompt()?;
        let commit_msg = Text::new("Commit message:").prompt()?;
        git_client.add_all_commit(branch_name.clone(), commit_msg.clone())?;
        git_client.push("create-pr".to_string())?;
        let pr_data = PrData::new(
            branch_name.clone(),
            commit_msg,
            branch_name,
            "main".to_string(),
        );
        gh_client.create_pull_request(org, repo, pr_data).await?;

        Ok("Ok".to_string())
    }
}
