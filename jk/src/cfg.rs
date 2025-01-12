//! Module for the jk configuration file.
use anyhow::{anyhow, Error, Result};
use config::Config;
use dirs::home_dir;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JkConfig {
    /// Filepath to a template for Git commits.
    pub git_commit_template: String,
    /// Filepath to the SSH key used to push to GitHub.
    pub ssh_key_fp: String,
}

impl JkConfig {
    /// Get the jk config file from the users home directory.
    pub fn get() -> Result<JkConfig> {
        if let Some(home) = home_dir() {
            let settings = Config::builder()
                .add_source(config::File::with_name(&format!(
                    "{}/.jk.toml",
                    home.to_string_lossy()
                )))
                .build()?;
            settings.try_deserialize::<JkConfig>().map_err(Error::from)
        } else {
            Err(anyhow!("HOME directory could not be found!"))
        }
    }
}
