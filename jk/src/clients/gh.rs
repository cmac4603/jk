use std::env;

use anyhow::{Error, Result};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, USER_AGENT},
    Client, Response,
};
use serde::Serialize;

pub struct GithubClient<'gh> {
    api_domain: &'gh str,
    client: Client,
    user_agent: &'gh str,
    token: Result<String>,
}

impl<'gh> Default for GithubClient<'gh> {
    fn default() -> Self {
        Self {
            api_domain: "https://api.github.com",
            client: Client::new(),
            user_agent: "jk-cli/v0",
            token: env::var("GITHUB_TOKEN").map_err(Error::from),
        }
    }
}

#[derive(Serialize)]
pub struct PrData {
    title: String,
    body: String,
    head: String,
    base: String,
}

impl PrData {
    pub fn new(head: String, body: String, base: String) -> Self {
        PrData {
            title: head.clone(),
            body,
            head,
            base,
        }
    }
}

impl<'gh> GithubClient<'gh> {
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.token.as_ref().unwrap())
                .parse()
                .unwrap(),
        );
        headers.insert(USER_AGENT, self.user_agent.parse().unwrap());
        headers
    }

    pub async fn create_pull_request(
        self,
        org: String,
        repo: String,
        data: PrData,
    ) -> Result<Response> {
        self.client
            .post(format!("{}/repos/{}/{}/pulls", self.api_domain, org, repo))
            .headers(self.get_headers())
            .json(&data)
            .send()
            .await
            .map_err(Error::from)
    }
}
