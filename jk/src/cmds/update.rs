//! Self-update the jk command from GitHub.
use std::env;

use anyhow::{anyhow, Result};
use self_update::cargo_crate_version;

pub async fn update() -> Result<String> {
    let gh_token = env::var("GITHUB_TOKEN").expect("Require GITHUB_TOKEN env var!");
    let status = self_update::backends::github::Update::configure()
        .repo_owner("cmac4603")
        .repo_name("jk")
        .bin_name("github")
        .show_download_progress(true)
        .auth_token(&gh_token)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    if status.uptodate() {
        Ok(format!("Already up-to-date at v{}", status.version()))
    } else if status.updated() {
        Ok(format!("Updated to v{}", status.version()))
    } else {
        Err(anyhow!("Unexpected update, not up-to-date or updated."))
    }
}
