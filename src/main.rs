#[macro_use] extern crate rocket;
extern crate chrono;
extern crate diesel;
use rest_backend::*;
use rocket::http::Status;
use rocket::serde::json::Json;

mod show_user;
use crate::show_user::*;
mod new_user;
use crate::new_user::insert_user;
use crate::models::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_users() -> Json<Vec<ResponseUser>> {
    Json(load_all())
}

#[get("/users/<id>")]
fn get_user(id: i32) -> Json<ResponseUser> {
    Json(load_user(id))
}

#[post("/users/new", data = "<new_user_data>")]
fn create_user(new_user_data: Json<NewUser>) -> String {
    insert_user(&mut new_user_data.into_inner());
    String::from("Created the new user!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/api", routes![
        get_users,
        get_user,
        create_user
    ])
}