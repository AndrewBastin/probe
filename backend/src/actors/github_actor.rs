use chrono::{DateTime, Utc};
use futures::future;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, error::Error};

pub struct GithubActor {
    client: reqwest::Client,
    github_api_token: String
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
    pub funding: Option<FundingYMLContent>
}

#[derive(Deserialize)]
struct UnghFileResponseFile {
    contents: String
}

#[derive(Deserialize)]
struct UnghFileResponse {
    file: UnghFileResponseFile
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundingYMLContent {
    #[serde(default)]
    pub github: Vec<String>,

    #[serde(default)]
    pub patreon: Option<String>,

    #[serde(default)]
    pub open_collective: Option<String>,

    #[serde(default)]
    pub ko_fi: Option<String>,

    #[serde(default)]
    pub tidelift: Option<String>,

    #[serde(default)]
    pub community_bridge: Option<String>,

    #[serde(default)]
    pub liberapay: Option<String>,

    #[serde(default)]
    pub issuehunt: Option<String>,

    #[serde(default)]
    pub otechie: Option<String>,

    #[serde(default)]
    pub custom: Vec<String>,
}

impl FundingYMLContent {
    pub fn to_url_map(&self) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();

        if !self.github.is_empty() {
            map.insert("github".to_string(), self.github.iter()
                .map(|username| format!("https://github.com/sponsors/{}", username))
                .collect());
        }

        if let Some(ref username) = self.patreon {
            map.insert("patreon".to_string(), vec![format!("https://www.patreon.com/{}", username)]);
        }

        if let Some(ref project) = self.open_collective {
            map.insert("open_collective".to_string(), vec![format!("https://opencollective.com/{}", project)]);
        }

        if let Some(ref username) = self.ko_fi {
            map.insert("ko_fi".to_string(), vec![format!("https://ko-fi.com/{}", username)]);
        }

        if let Some(ref package_name) = self.tidelift {
            map.insert("tidelift".to_string(), vec![format!("https://tidelift.com/funding/github/{}", package_name)]);
        }

        if let Some(ref project_id) = self.community_bridge {
            map.insert("community_bridge".to_string(), vec![format!("https://funding.communitybridge.org/projects/{}", project_id)]);
        }

        if let Some(ref username) = self.liberapay {
            map.insert("liberapay".to_string(), vec![format!("https://liberapay.com/{}", username)]);
        }

        if let Some(ref project) = self.issuehunt {
            map.insert("issuehunt".to_string(), vec![format!("https://issuehunt.io/r/{}", project)]);
        }

        if let Some(ref username) = self.otechie {
            map.insert("otechie".to_string(), vec![format!("https://otechie.com/{}", username)]);
        }

        if !self.custom.is_empty() {
            map.insert("custom".to_string(), self.custom.clone());
        }

        map
    }
}

impl GithubActor {
    pub fn new(github_api_token: String) -> Self {
        Self {
            client: Client::new(),
            github_api_token
        }
    }

    pub async fn get_github_info(&self, repo: &str) -> Result<GithubInfo, Box<dyn Error>> {
        println!("Getting GitHub info for: {}", repo);

        let base_url = "https://api.github.com";

        // Get repository information
        let repo_url = format!("{}/repos/{}", base_url, repo);
        let repo_info: Value = self.client.get(&repo_url)
            .header("User-Agent", "probe")
            .header("Authorization", format!("token {}", self.github_api_token))
            .send()
            .await?
            .json()
        .await?;

        println!("Repo Info");

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

        println!("Contributors Info");

        // Get issues
        let issues_url = format!("{}/repos/{}/issues?state=closed", base_url, repo);
        let closed_issues: Vec<Value> = self.client.get(&issues_url)
            .header("User-Agent", "probe")
            .header("Authorization", format!("token {}", self.github_api_token))
            .send()
            .await?
            .json()
        .await?;

        println!("Issues Info");

        // Calculate average issue closing time
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

        println!("Closing time Info");

        // Get last commit date
        let commits_url = format!("{}/repos/{}/commits", base_url, repo);
        let commits: Vec<Value> = self.client.get(&commits_url)
            .header("User-Agent", "probe")
            .header("Authorization", format!("token {}", self.github_api_token))
            .send()
            .await?
            .json()
        .await?;

        println!("Commits time Info");

        let last_commit_at = DateTime::parse_from_rfc3339(commits[0]["commit"]["committer"]["date"].as_str().unwrap())?;

        println!("Last Commit At");

        Ok(GithubInfo {
            repo_url: repo_info["html_url"].as_str().unwrap().to_string(),
            repo_desc: repo_info["description"].as_str().unwrap_or("").to_string(),
            license_key: repo_info["license"]["key"].as_str().map(|s| s.to_string()),
            no_of_contributors: contributors.len(),
            total_issues: closed_issues.len(),
            open_issues: repo_info["open_issues_count"].as_u64().unwrap() as usize,
            avg_issue_closing_time_mins: avg_closing_time,
            last_commit_at: last_commit_at.with_timezone(&Utc),
            funding: self.get_funding_links(repo).await
        })
    }

    pub async fn get_funding_links(&self, repo: &str) -> Option<FundingYMLContent> {
        println!("Getting funding links for: {}", repo);

        let repo_info_link = format!("https://ungh.cc/repos/{}", repo);

        let funding_info: Value = self.client.get(&repo_info_link)
            .header("User-Agent", "probe")
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()?;

        let default_branch = funding_info
            .as_object()?
            ["repo"]
            .as_object()?
            ["defaultBranch"]
            .as_str()
            .unwrap_or("master");


        let funding_yml_urls = vec![
            format!("https://ungh.cc/repos/{}/files/{}/FUNDING.yml", repo, default_branch),
            format!("https://ungh.cc/repos/{}/files/{}/.github/FUNDING.yml", repo, default_branch),
        ];

        let sponsorship_futures = funding_yml_urls.into_iter().map(|url| {
            async move {
                let response = match self.client.get(&url).send().await {
                    Ok(response) => response,
                    Err(_) => return None
                };

                let json = match response.json::<UnghFileResponse>().await {
                    Ok(data) => data,
                    Err(_) => return None
                };

                let funding_info: FundingYMLContent = match serde_yaml::from_str(&json.file.contents) {
                    Ok(data) => data,
                    Err(_) => return None
                };


                Some(funding_info)
            }
        });

        future::join_all(sponsorship_futures).await
            .into_iter()
            .find(|result| result.is_some())
            .map(|result| result.unwrap())
    }

}
