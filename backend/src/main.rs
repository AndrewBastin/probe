use controllers::AppState;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;
use models::DB;

#[macro_use] extern crate rocket;

mod actors;
mod models;
mod controllers;
mod utils;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(DB::init())
        .attach(
            AdHoc::on_ignite("DB Migrator", |rocket| Box::pin(async move {
                let db = DB::fetch(&rocket)
                    .expect("[Migration] Failed to get database");

                sqlx::migrate!().run(&**db)
                    .await
                    .expect("[Migration] Error while running migration");

                rocket
            }))
        )
        .manage(AppState::default())
        .mount("/", controllers::routes.clone())
        .launch()
        .await;
}
