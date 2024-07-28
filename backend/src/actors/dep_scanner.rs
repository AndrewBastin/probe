use std::{collections::{HashMap, VecDeque}, sync::Arc};
use tokio::sync::{RwLock, Semaphore};
use futures::{future::join_all, stream::iter, StreamExt};
use crate::actors::npm_actor::PackageInfo;

use super::{ai_actor::{AIActor, ParsedFundingInfo}, github_actor::{GithubActor, GithubInfo}, npm_actor::NPMActor};

#[derive(Clone)]
pub enum DepScannerStatus {
    Processing,
    Done {
        dep_objects: HashMap<(String, String), PackageInfo>,
        deps_on_graph: HashMap<Option<(String, String)>, Vec<(String, String)>>,
        github_info_map: HashMap<(String, String), GithubInfo>,
        funding_info_map: HashMap<String, Option<ParsedFundingInfo>>,
    },
}

pub struct DepScannerActor {
    id: String,
    status: RwLock<DepScannerStatus>,
}

fn get_github_repo_from_package_url(url: &str) -> Option<String> {
    // Remove the protocol prefix if present
    let cleaned_url = url.trim_start_matches("git+https://")
        .trim_start_matches("https://")
        .trim_start_matches("http://");

    // Check if it's a GitHub URL
    if !cleaned_url.starts_with("github.com/") {
        return None;
    }

    // Extract the repo name
    let repo_name = cleaned_url
        .trim_start_matches("github.com/")
        .trim_end_matches(".git")
        .to_string();

    // Ensure we have a non-empty string
    if repo_name.is_empty() {
        None
    } else {
        Some(repo_name)
    }
}

impl DepScannerActor {
    async fn get_dep_graph(
        npm_actor: Arc<NPMActor>,
        package_json: serde_json::Value
    ) -> (
        HashMap<(String, String), PackageInfo>, 
        HashMap<Option<(String, String)>, Vec<(String, String)>>
    ) {
        let mut dep_objects: HashMap<(String, String), PackageInfo> = HashMap::new();
        let mut deps_on_graph: HashMap<Option<(String, String)>, Vec<(String, String)>> = HashMap::new();
        let mut queue: VecDeque<(String, String, Option<(String, String)>)> = VecDeque::new();

        let dependencies = package_json["dependencies"].as_object()
            .unwrap_or(&serde_json::Map::new())
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            .collect::<Vec<_>>();

        deps_on_graph.insert(None, dependencies.clone());

        // Initialize queue with top-level dependencies
        for (name, version) in &dependencies {
            queue.push_back((name.clone(), version.clone(), None));
        }

        let semaphore = Arc::new(Semaphore::new(5)); // Limit to 5 concurrent fetches
        let mut total_processed = 0;

        println!("Starting dependency scan. Initial queue size: {}", queue.len());

        while !queue.is_empty() {
            let mut futures = Vec::new();

            for _ in 0..5 {
                if let Some((name, version, parent)) = queue.pop_front() {
                    let npm_actor_clone = npm_actor.clone();
                    let semaphore_clone = semaphore.clone();

                    futures.push(tokio::spawn(async move {
                        let _permit = semaphore_clone.acquire().await.unwrap();
                        let details = npm_actor_clone.get_package_info(&name, &version).await;
                        (name, version, parent, details)
                    }));
                } else {
                    break;
                }
            }

            let results = join_all(futures).await;

            for result in results {
                if let Ok((name, version, parent, details)) = result {
                    total_processed += 1;
                    println!("Processing dependency {}: {}@{} (Queue size: {})", 
                        total_processed, name, version, queue.len());

                    if dep_objects.contains_key(&(name.clone(), version.clone())) {
                        continue;
                    }

                    if let Some(details) = details {
                        dep_objects.insert((name.clone(), version.clone()), details.clone());

                        let sub_deps = details.dependencies.iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect::<Vec<_>>();

                        deps_on_graph.insert(
                            Some((name.clone(), version.clone())),
                            sub_deps.clone()
                        );

                        for (sub_name, sub_version) in sub_deps {
                            queue.push_back((
                                sub_name,
                                sub_version,
                                Some((name.clone(), version.clone()))
                            ));
                        }

                        if let Some(parent) = parent {
                            if let Some(parent_deps) = deps_on_graph.get_mut(&Some(parent)) {
                                parent_deps.push((name.clone(), version.clone()));
                            }
                        }
                    } else {
                        println!("Processing failed for {}@{}", name, version);
                    }
                }
            }
        }

        return (dep_objects, deps_on_graph);
    }

    pub fn start(
        id: String,
        package_json: &serde_json::Value,
        npm_actor: Arc<NPMActor>,
        github_actor: Arc<GithubActor>,
        ai_actor: Arc<AIActor>
    ) -> Arc<DepScannerActor> {
        let state = Arc::new(
            DepScannerActor {
                id,
                status: RwLock::new(DepScannerStatus::Processing),
            }
        );

        let moved_state = state.clone();
        let package_json = package_json.clone();

        // Main processing routine
        tokio::spawn(async move {
            let (dep_objects, deps_on_graph) = DepScannerActor::get_dep_graph(npm_actor, package_json).await;

            let repo_urls = dep_objects
                .iter()
                .filter_map(|(key, pkg)| {
                    if let Some(repo) = &pkg.repository {
                        if repo.repo_type == "git" {
                            if let Some(repo_url) = get_github_repo_from_package_url(&repo.url) {
                                Some((key.clone(), repo_url))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<HashMap<(String, String), String>>();

            // let mut github_info_map = HashMap::new();
            //
            // for (key, repo_url) in repo_urls.iter() {
            //     let github_info = github_actor.get_github_info(&repo_url).await.unwrap();
            //
            //     github_info_map.insert(key.clone(), github_info);
            // }
            //
            // let mut funding_info_map: HashMap<String, Option<ParsedFundingInfo>> = HashMap::new();
            //
            // for gh_info in github_info_map.values() {
            //     if let Some(info) = &gh_info.funding {
            //         for domain_urls in info.to_url_map().values() {
            //             for url in domain_urls {
            //                 if let Some(summary) = ai_actor.summarize_page(url).await {
            //                     let data = ai_actor.get_funding_stats_from_page_summary(url, &summary).await.ok();
            //
            //                     funding_info_map.insert(url.clone(), data);
            //                 } else {
            //                     funding_info_map.insert(url.clone(), None);
            //                 }
            //             }
            //         }
            //
            //     }
            // }


            let mut github_info_futures = Vec::new();

            for (key, repo_url) in repo_urls.iter() {
                let github_actor = github_actor.clone();
                let key = key.clone();
                let repo_url = repo_url.clone();
                github_info_futures.push(async move {
                    let github_info = github_actor.get_github_info(&repo_url).await.unwrap();
                    (key, github_info)
                });
            }

            let github_info_map: HashMap<_, _> = iter(github_info_futures)
                .buffer_unordered(5)
                .collect()
            .await;

            let mut funding_info_futures = Vec::new();

            for gh_info in github_info_map.values() {
                if let Some(info) = &gh_info.funding {
                    for domain_urls in info.to_url_map().values() {
                        for url in domain_urls {
                            let ai_actor = ai_actor.clone();
                            let url = url.clone();
                            funding_info_futures.push(async move {
                                if let Some(summary) = ai_actor.summarize_page(&url).await {
                                    let data = ai_actor.get_funding_stats_from_page_summary(&url, &summary).await.ok();
                                    (url, data)
                                } else {
                                    (url, None)
                                }
                            });
                        }
                    }
                }
            }

            let funding_info_map: HashMap<_, _> = iter(funding_info_futures)
                .buffer_unordered(5)
                .collect()
            .await;

            *moved_state.status.write().await = DepScannerStatus::Done {
                dep_objects,
                deps_on_graph,
                github_info_map,
                funding_info_map
            };
        });

        state
    }

    pub async fn get_status(self: &Arc<Self>) -> DepScannerStatus {
        self.status.read().await.clone()
    }
}
