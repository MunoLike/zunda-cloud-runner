use axum::{extract::Path, routing::get, Json, Router};

use serde::Serialize;
use std::net::SocketAddr;
use uuid::Uuid;

mod bindings;
use bindings::*;

#[derive(Serialize, Debug)]
struct User {
    id: Uuid,
    username: String,
}

async fn get_user() -> Json<User> {
    let user = User {
        id: Uuid::new_v4(),
        username: String::from("aaaaaaaa"),
    };
    println!("{:?}", user);
    Json(user)
}

#[tokio::main]
async fn main() {
    unsafe {
        let core = voicevox_core::new(String::from("./libvoicevox_core.so")).unwrap();
    };

    let app = Router::new().route("/", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");
}
