#[macro_use] extern crate rocket;
extern crate chrono;
extern crate diesel;
use rest_backend::*;
use rocket::Request;
use rocket::http::Status;
use rocket::serde::json::Json;

mod show_user;
use crate::show_user::*;
mod new_user;
use crate::new_user::insert_user;
use crate::models::*;

mod show_post;
use crate::show_post::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_users() -> Json<Vec<ResponseUser>> {
    Json(show_user::load_all())
}

#[get("/users/<id>")]
fn get_user(id: i32) -> Json<ResponseUser> {
    Json(load_user(id))
}

#[catch(500)]
fn error_catcher(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path", req.uri())
}

#[post("/users/new", data = "<new_user_data>")]
fn create_user(new_user_data: Json<NewUser>) -> String {
    insert_user(&mut new_user_data.into_inner());
    String::from("Created the new user!")
}

#[get("/posts")]
fn get_posts() -> Json<Vec<ResponsePost>> {
    Json(show_post::load_all())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/api", routes![
        get_users,
        get_user,
        create_user,
        get_posts
    ])
    .register("/", catchers![error_catcher])
}