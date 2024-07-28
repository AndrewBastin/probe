use chrono::{DateTime, Utc};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct GithubActor {
    client: reqwest::Client
}

#[derive(Clone, Debug)]
pub struct GithubInfo {
    pub repo_url: String,
    pub repo_desc: String,
    pub license_key: Option<String>,
    pub no_of_contributors: usize,
    pub total_issues: usize,
    pub open_issues: usize,
    pub avg_issue_closing_time_mins: f64,
    pub last_commit_at: DateTime<Utc>,
}

impl Default for GithubActor {
    fn default() -> Self {
        Self {
            client: Client::new()
        }
    }
}

impl GithubActor {
    pub async fn get_github_info(&self, repo: &str) -> Result<GithubInfo, Box<dyn Error>> {
        println!("Getting GitHub info for: {}", repo);

        let base_url = "https://api.github.com";

        // Get repository information
        let repo_url = format!("{}/repos/{}", base_url, repo);
        let repo_info: Value = self.client.get(&repo_url)
            .header("User-Agent", "probe")
            .send()
            .await?
            .json()
        .await?;

        // Get contributors count
        let contributors_url = format!("https://ungh.cc/repos/{}/contributors", repo);
        let contributors: Vec<Value> = self.client.get(&contributors_url)
            .header("User-Agent", "probe")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?
            .as_object()
            .unwrap()
            ["contributors"]
            .as_array()
            .unwrap()
            .to_owned();

        // Get issues
        let issues_url = format!("{}/repos/{}/issues?state=all", base_url, repo);
        let issues: Vec<Value> = self.client.get(&issues_url)
            .header("User-Agent", "probe")
            .send()
            .await?
            .json()
        .await?;

        // Calculate average issue closing time
        let closed_issues: Vec<&Value> = issues.iter().filter(|i| i["state"] == "closed").collect();
        let total_closing_time = closed_issues.iter().fold(0.0, |acc, issue| {
            let created_at = DateTime::parse_from_rfc3339(issue["created_at"].as_str().unwrap()).unwrap();
            let closed_at = DateTime::parse_from_rfc3339(issue["closed_at"].as_str().unwrap()).unwrap();
            acc + (closed_at - created_at).num_minutes() as f64
        });
        let avg_closing_time = if !closed_issues.is_empty() {
            total_closing_time / closed_issues.len() as f64
        } else {
            0.0
        };

        // Get last commit date
        let commits_url = format!("{}/repos/{}/commits", base_url, repo);
        let commits: Vec<Value> = self.client.get(&commits_url)
            .header("User-Agent", "probe")
            .send()
            .await?
            .json()
        .await?;
        let last_commit_at = DateTime::parse_from_rfc3339(commits[0]["commit"]["committer"]["date"].as_str().unwrap())?;

        Ok(GithubInfo {
            repo_url: repo_info["html_url"].as_str().unwrap().to_string(),
            repo_desc: repo_info["description"].as_str().unwrap_or("").to_string(),
            license_key: repo_info["license"]["key"].as_str().map(|s| s.to_string()),
            no_of_contributors: contributors.len(),
            total_issues: issues.len(),
            open_issues: repo_info["open_issues_count"].as_u64().unwrap() as usize,
            avg_issue_closing_time_mins: avg_closing_time,
            last_commit_at: last_commit_at.with_timezone(&Utc),
        })
    }

}
