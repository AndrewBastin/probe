use controllers::DummyRoutesState;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;
use models::DB;

#[macro_use] extern crate rocket;

mod models;
mod controllers;

#[get("/")]
fn index() -> &'static str {
    "Hello Everynyan!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
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
        .manage(DummyRoutesState::default())
        .mount("/", controllers::routes.clone())
}
