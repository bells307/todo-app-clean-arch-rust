mod todo;

use crate::application::todo::TodoService;
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;

pub fn todo_router() -> Router {
    Router::new()
        .route("/todo", post(todo::create_todo))
        .route("/todo", get(todo::get_all_todos))
        .route("/todo/:id", get(todo::get_todo))
        .route("/todo/:id/done", put(todo::todo_done))
        .route("/todo/:id", delete(todo::delete_todo))
}
