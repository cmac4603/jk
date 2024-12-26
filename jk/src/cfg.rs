use std::env;

use anyhow::{anyhow, Error, Result};
use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JkConfig {
    pub github_email: String,
    pub github_username: String,
    pub ssh_key_fp: String,
}

impl JkConfig {
    pub fn get() -> Result<JkConfig> {
        if let Some(home) = env::home_dir() {
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
