use anyhow::Result;
use clap::Parser;

use crate::{cfg::JkConfig, clients::gh::GithubClient, tools::git::GitClient};

#[derive(Debug, Parser)]
pub struct NewArgs {}

#[derive(Debug, Parser)]
pub enum ManageDependabot {
    /// Create a new branch with all open dependabot PRs
    New(NewArgs),
}

impl ManageDependabot {
    pub async fn run_cmd(self, cfg: JkConfig) -> Result<()> {
        match self {
            Self::New(ref _args) => self.create_dependabot_branch(cfg).await,
        }
    }

    pub async fn create_dependabot_branch(self, cfg: JkConfig) -> Result<()> {
        let git_client = GitClient::new(cfg)?;
        let gh_client = GithubClient::default();
        let (org, repo) = git_client.get_org_repo()?;
        let items = gh_client
            .list_prs_by_assignee(org, repo, "dependabot%5Bbot%5D")
            .await?;
        println!("{:?}", items);
        Ok(())
    }
}
