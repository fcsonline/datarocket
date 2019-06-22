#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::sync::atomic::{AtomicUsize, Ordering};
use rocket_contrib::json::Json;
use rocket::State;

struct HitCount {
    count: AtomicUsize
}

#[derive(Serialize, Deserialize)]
struct User {
  id: String,
  first_name: String,
  last_name: String,
  nick: String
}

#[get("/users", format = "json")]
fn users_index(hit_count: State<HitCount>) -> Json<Vec<User>> {
    let count = hit_count.count.load(Ordering::Relaxed);

    let user1 = User{
        id: count.to_string(),
        first_name: count.to_string(),
        last_name: count.to_string(),
        nick: count.to_string(),
    };

    let user2 = User{
        id: count.to_string(),
        first_name: count.to_string(),
        last_name: count.to_string(),
        nick: count.to_string(),
    };

    let users = vec![user1, user2];

    Json(users)
}


#[post("/users", format = "json", data = "<user>")]
fn users_create(hit_count: State<HitCount>, user: Json<User>) -> Json<User> {
    hit_count.count.fetch_add(1, Ordering::Relaxed);

    user
}

fn main() {
    rocket::ignite()
        .manage(HitCount { count: AtomicUsize::new(0) })
        .mount("/", routes![
               users_index,
               users_create
        ])
        .launch();
}
