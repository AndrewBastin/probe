use lazy_static::lazy_static;
use rocket::Route;

#[get("/")]
fn index() -> &'static str {
    "Hello Everynyan!"
}

lazy_static! {
    pub static ref routes: Vec<Route> = routes![
        index,
    ];
}
