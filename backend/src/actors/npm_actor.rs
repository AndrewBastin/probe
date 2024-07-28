use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

pub struct NPMActor {
    client: reqwest::Client
}

impl Default for NPMActor {
    fn default() -> Self {
        Self {
            client: Client::new()
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Repository {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub url: String
}

#[derive(Clone, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub license: Option<String>,

    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    pub repository: Option<Repository>
}

impl NPMActor {
    // TODO: Make the type more solid
    pub async fn get_package_info(&self, package_name: &str, version: &str) -> Option<PackageInfo> {
        println!("Getting package info for {}@{}", package_name, version);

        // TODO: Make this respect version
        let npm_url = format!("https://registry.npmjs.org/{}/{}/", package_name, "latest");

        self.client.get(&npm_url).send().await
            .ok()?
            .json()
            .await
            .ok()?
    }
}
