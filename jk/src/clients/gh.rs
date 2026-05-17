use std::{env, fmt::Display, future::Future};

use anyhow::{Error, Result};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, USER_AGENT},
    Client, Error as ReqwestError, Response,
};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct CreatePrResponse {
    pub html_url: String,
}

#[derive(Deserialize)]
pub struct SearchResults {
    total_count: u64,
    incomplete_results: bool,
    pub items: Vec<SearchResultsItems>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResultsItems {
    pub number: u64,
}

#[derive(Deserialize)]
pub struct GetPrResponse {
    pub title: String,
    pub head: GetPrHead,
}

impl Display for GetPrResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | [{}]", self.title, self.head.ref_)
    }
}

#[derive(Deserialize)]
pub struct GetPrHead {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub sha: String,
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
    ) -> Result<CreatePrResponse> {
        let resp = self
            .client
            .post(format!("{}/repos/{}/{}/pulls", self.api_domain, org, repo))
            .headers(self.get_headers())
            .json(&data)
            .send()
            .await?;
        resp.error_for_status()?
            .json::<CreatePrResponse>()
            .await
            .map_err(Error::from)
    }

    pub async fn list_prs_by_assignee(
        &self,
        org: &String,
        repo: &String,
        assignee: &str,
    ) -> Result<Vec<SearchResultsItems>> {
        // curl -s "https://api.github.com/search/issues?q=is:open%20is:pr%20assignee:dhh%20repo:rails/rails"
        let search_q = format!(
            "is:open%20is:pr%20author:{}%20repo:{}/{}",
            assignee, org, repo
        );

        let resp = self
            .client
            .get(format!("{}/search/issues?q={}", self.api_domain, search_q))
            // .query(&["q", search_q.as_str()])
            .headers(self.get_headers())
            .send()
            .await?;
        resp.error_for_status()?
            .json::<SearchResults>()
            .await
            .map(|r| r.items)
            .map_err(Error::from)
    }

    pub fn get_pull_request(
        &self,
        org: &String,
        repo: &String,
        number: u64,
    ) -> Box<impl Future<Output = Result<Response, ReqwestError>>> {
        Box::new(self.client
            .get(format!(
                "{}/repos/{}/{}/pulls/{}",
                self.api_domain, org, repo, number
            ))
            // .query(&["q", search_q.as_str()])
            .headers(self.get_headers())
            .send()
        )
    }
}
