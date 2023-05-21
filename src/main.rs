#[macro_use] extern crate rocket;

use rocket::serde::json::{json, Value};

mod models;
mod routes;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
    .mount("/api", routes![
        routes::books::get_books,
        routes::books::get_book_by_id,
        routes::books::create_book
    ])
    .register("/", catchers![not_found])
}


