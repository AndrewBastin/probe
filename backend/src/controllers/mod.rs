use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use serde::Serialize;
use tokio::sync::RwLock;

use lazy_static::lazy_static;
use rocket::{form::Form, http::Status, serde::json::Json, Route, State};
use uuid::Uuid;

use crate::{actors::{dep_scanner::{DepScannerActor, DepScannerStatus}, github_actor::{FundingYMLContent, GithubActor}, npm_actor::NPMActor}, utils::JsonValue};

#[get("/")]
fn index() -> &'static str {
    "Hello Everynyan!"
}

pub struct AppState {
    pub jobs: Arc<RwLock<HashMap<String, Arc<DepScannerActor>>>>,
    pub npm_actor: Arc<NPMActor>,
    pub github_actor: Arc<GithubActor>
}

#[derive(FromForm)]
struct ProcessFileInput {
    file: JsonValue
}

#[derive(Serialize)]
struct ProcessFileOutput {
    tracking_id: String
}

#[post("/process_file", data = "<input>")]
async fn process_file(input: Form<ProcessFileInput>, state: &State<AppState>) -> Json<ProcessFileOutput> {
    let id = Uuid::new_v4().to_string();

    let actor = DepScannerActor::start(id.clone(), &input.file.0, state.npm_actor.clone(), state.github_actor.clone());

    let mut jobs = state.jobs.write().await;
    jobs.insert(id.clone(), actor);

    Json(ProcessFileOutput { tracking_id: id.to_string() })
}

#[derive(Serialize)]
struct GetStatusResult {
    loading: bool
}

#[get("/status?<id>")]
async fn get_status(id: String, state: &State<AppState>) -> Result<Json<GetStatusResult>, Status> {
    let jobs = state.jobs.read().await;

    let job = if let Some(job) = jobs.get(&id) { job } else { return Err(Status::NotFound) };

    match job.get_status().await {
        DepScannerStatus::Processing => Ok(Json(GetStatusResult { loading: true })),
        DepScannerStatus::Done { dep_objects: _, deps_on_graph: _, github_info_map: _ } => 
            Ok(Json(GetStatusResult { loading: false })),
    }
}

#[derive(Serialize)]
struct GithubFundingInfo {
    links: HashMap<String, Vec<String>>
}

#[derive(Serialize)]
struct GithubInfo {
    repo_url: String,
    repo_desc: String,
    no_of_contributors: u32,
    total_issues: usize,
    open_issues: usize,
    avg_issue_closing_time_mins: f64,
    last_commit_at: chrono::DateTime<Utc>,
    funding: Option<GithubFundingInfo>
}

#[derive(Serialize)]
struct Dependency {
    name: String,
    license: Option<String>,
    github: Option<GithubInfo>,
}

#[derive(Serialize)]
struct GetResultResultV2 {
    objects: Vec<Dependency>,
    root_objects: Vec<String>,
    depends_on: HashMap<String, Vec<String>>
}

#[get("/result?<id>")]
async fn get_result(id: String, state: &State<AppState>) -> Result<Json<GetResultResultV2>, Status> {
    let jobs = state.jobs.read().await;

    let job = if let Some(job) = jobs.get(&id) { job } else { return Err(Status::NotFound) };

    match job.get_status().await {
        DepScannerStatus::Done { dep_objects, deps_on_graph, github_info_map } => {
            let objects: Vec<Dependency> = dep_objects
                .iter()
                .map(|(key, pkg)| {
                    let github_info = if let Some(repo_url) = pkg.repository.as_ref().and_then(|repo| {
                        if repo.repo_type == "git" {
                            Some(&repo.url)
                        } else {
                            None
                        }
                    }) {
                        github_info_map.get(key).cloned()
                    } else {
                        None
                    };

                    Dependency {
                        name: format!("{}@{}", key.0, key.1),
                        license: pkg.license.clone(),
                        github: github_info 
                            .map(|info| {
                                GithubInfo {
                                    repo_url: info.repo_url,
                                    repo_desc: info.repo_desc,
                                    open_issues: info.open_issues,
                                    total_issues: info.total_issues,
                                    last_commit_at: info.last_commit_at,
                                    no_of_contributors: info.no_of_contributors,
                                    avg_issue_closing_time_mins: info.avg_issue_closing_time_mins,
                                    funding: info.funding.map(|f| GithubFundingInfo { links: f.to_url_map() })
                                }
                            })
                    }
                }).collect();

            let root_objects = deps_on_graph
                .get(&None)
                .map(|deps| {
                    deps
                        .iter()
                        .map(|(name, version)| format!("{}@{}", name, version))
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default();

            let depends_on: HashMap<String, Vec<String>> = deps_on_graph
                .iter()
                .filter_map(|(key, deps)| {
                    if let Some(key) = key {
                        let entries = deps.iter().map(|(name, version)|
                            format!("{}@{}", name, version)
                        ).collect::<Vec<String>>();

                        Some((format!("{}@{}", key.0, key.1), entries))
                    } else {
                        None
                    }
                })
                .collect();

            Ok(Json(GetResultResultV2 { 
                objects,
                root_objects,
                depends_on
            }))
        },
        DepScannerStatus::Processing => Err(Status::BadRequest)
    }
}

lazy_static! {
    pub static ref routes: Vec<Route> = routes![
        index,

        process_file,
        get_status,
        get_result
    ];
}
