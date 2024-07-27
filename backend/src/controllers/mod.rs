use std::{collections::HashMap, sync::Arc, time::Duration};

use serde::Serialize;
use tokio::sync::RwLock;

use lazy_static::lazy_static;
use rocket::{form::Form, fs::TempFile, http::Status, serde::json::Json, Route, State};
use uuid::Uuid;

#[get("/")]
fn index() -> &'static str {
    "Hello Everynyan!"
}

#[derive(Debug)]
enum ProcessState {
    Processing,
    Done // TODO: Data
}

#[derive(Default)]
pub struct DummyRoutesState {
    issued_ids: Arc<RwLock<HashMap<String, ProcessState>>>
}

#[derive(FromForm)]
struct ProcessFileInput<'r> {
    file: TempFile<'r>
}

#[derive(Serialize)]
struct ProcessFileOutput {
    tracking_id: String
}

#[post("/process_file", data = "<input>")]
async fn process_file(input: Form<ProcessFileInput<'_>>, state: &State<DummyRoutesState>) -> Json<ProcessFileOutput> {
    let mut issued_ids = state.issued_ids.write().await;

    let id = Arc::new(Uuid::new_v4().to_string());

    issued_ids.insert(id.to_string(), ProcessState::Processing);

    drop(issued_ids);

    let moved_id = id.clone();
    let moved_issued_ids = state.issued_ids.clone();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(10)).await;

        let mut running_processes = moved_issued_ids.write().await;

        *running_processes.get_mut(&*moved_id).unwrap() = ProcessState::Done;

    });


    Json(ProcessFileOutput { tracking_id: id.to_string() })
}

#[derive(Serialize)]
struct GetStatusResult {
    loading: bool
}

#[get("/status?<id>")]
async fn get_status(id: String, state: &State<DummyRoutesState>) -> Result<Json<GetStatusResult>, Status> {
    let issued_ids = state.issued_ids.read().await;

    dbg!(id.clone());
    println!("{:?}", issued_ids);

    match issued_ids.get(&id) {
        Some(ProcessState::Done) => Ok(Json(GetStatusResult { loading: false })),
        Some(_) => Ok(Json(GetStatusResult { loading: true })),
        None => Err(Status::NotFound),
    }
}

#[derive(Serialize)]
struct GetResultResult {
    id: String
}

#[get("/result?<id>")]
async fn get_result(id: String, state: &State<DummyRoutesState>) -> Result<Json<GetResultResult>, Status> {
    let issued_ids = state.issued_ids.read().await;

    match issued_ids.get(&id) {
        Some(ProcessState::Done) => Ok(Json(GetResultResult { id: id.clone() })),
        Some(_) => Err(Status::BadRequest),
        None => Err(Status::NotFound),
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
