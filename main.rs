use axum::{
    extract::Json,
    // response::Html,
    routing::{post},
    Router,
};
use std::process::Command;
use std::thread;
use serde::Deserialize;

#[derive(Deserialize)]
struct Item {
   pub value: String
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", post(create_item));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_item(Json(body): Json<Item>) {
    let url = body.value.to_string();
    thread::spawn(move || {
        let _child = Command::new("cmd.exe")
            .arg("/C")
            .arg("ping")
            .arg(&url) // CWE-78: OS Command Injection-- Runs arbitrary command using the permissions of the user that is running the Rust program!
            .arg("-t")
            .spawn()
            .expect("Couldn't run &arg");
    });
}
