mod adapters;
mod application;
mod domain;
mod infrastructure;

use crate::adapters::repository::todo::mem::TodoMemoryRepository;
use crate::adapters::repository::todo::mongodb::TodoMongoDBRepository;
use crate::application::todo::TodoService;
use crate::infrastructure::delivery::http::axum::todo_router;
use axum::{Extension, Router};
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::sync::Arc;

const LISTEN_ADDR: &str = "127.0.0.1:9999";

#[tokio::main]
async fn main() {
    let mongo_client_options = ClientOptions::parse("mongodb://admin:admin@localhost:27017")
        .await
        .unwrap();
    let mongo_client = Client::with_options(mongo_client_options).unwrap();
    let mongo_db = mongo_client.database("todo-app");

    let todo_repo = TodoMongoDBRepository::new(mongo_db);
    let todo_service = TodoService::new(Box::new(todo_repo));

    let app = Router::new()
        .nest("/api", todo_router())
        .layer(Extension(Arc::new(todo_service)));

    println!("start listening on {LISTEN_ADDR} ...");

    axum::Server::bind(&LISTEN_ADDR.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
