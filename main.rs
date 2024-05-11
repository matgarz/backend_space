#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("Hello, world!")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .launch();
}

#[get("/custom-message")]
fn custom_message() -> Json<serde_json::Value> {
    let message = json!({"message": "This is a custom message"});
    Json(message)
}

#[get("/hello/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello, {}!", name)
}
#[get("/hello-query?<name>")]
fn hello_query(name: Option<String>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name),
        None => "Hello, anonymous!".to_string(),
    }
}

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[post("/user", format = "json", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    println!("Received user: {:?}", user);
    user
}
#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[post("/user", format = "json", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    println!("Received user: {:?}", user);
    user
}

