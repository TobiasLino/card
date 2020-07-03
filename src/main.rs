#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

use model::{user::User, address::Address};

#[post("/", data = "<user>")]
fn create(user: Json<User>) -> Json<User> {
    user
}

#[get("/")]
fn read() -> Json<JsonValue> {
    Json(json!([
        "user 1",
        "user 2"
    ]))
}

#[put("/<username>", data = "<user>")]
fn update(username: String, user: Json<User>) -> Json<User> {
    user
}

#[delete("/<username>")]
fn delete(username: String) -> Json<JsonValue> {
    Json(json!({ "status": "ok" }))
}

fn main() {
    rocket::ignite()
        .mount("/user", routes![create, update, delete])
        .mount("/users", routes![read])
        .launch();
}