use crate::application::todo::dto::CreateTodo;
use crate::application::todo::TodoService;
use crate::domain::todo::error::TodoError;
use crate::domain::todo::Todo;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

/// Создать задачу
pub async fn create_todo(
    todo_service: Extension<Arc<TodoService>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    todo_service
        .create(payload)
        .await
        .map(Json::from)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Получить все задачи
pub async fn get_all_todos(todo_service: Extension<Arc<TodoService>>) -> impl IntoResponse {
    todo_service
        .get_all()
        .await
        .map(Json::from)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Получить задачу по ID
pub async fn get_todo(
    todo_service: Extension<Arc<TodoService>>,
    Path(todo_id): Path<String>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo_id =
        Uuid::parse_str(&todo_id).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    todo_service
        .get(todo_id)
        .await
        .map(Json::from)
        .map_err(|e| match e {
            TodoError::NotFound(_) => (StatusCode::NOT_FOUND, "".into()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })
}

/// Пометить задачу как сделанную
pub async fn todo_done(
    todo_service: Extension<Arc<TodoService>>,
    Path(todo_id): Path<String>,
) -> Result<(), (StatusCode, String)> {
    let todo_id =
        Uuid::parse_str(&todo_id).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    todo_service.mark_done(todo_id).await.map_err(|e| match e {
        TodoError::NotFound(_) => (StatusCode::NOT_FOUND, "".into()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    })
}

/// Удалить задачу
pub async fn delete_todo(
    todo_service: Extension<Arc<TodoService>>,
    Path(todo_id): Path<String>,
) -> Result<(), (StatusCode, String)> {
    let todo_id =
        Uuid::parse_str(&todo_id).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    todo_service.delete(todo_id).await.map_err(|e| match e {
        TodoError::NotFound(_) => (StatusCode::NOT_FOUND, "".into()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    })
}
