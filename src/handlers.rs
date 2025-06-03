use crate::{
    db,
    errors::AppError,
    models::{CreateTodo, Todo},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::SqlitePool;

pub async fn get_todos(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = db::get_all_todos_db(&pool).await?;
    Ok(Json(todos))
}

pub async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(new_todo): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let todo = db::create_todo_db(&pool, new_todo).await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, AppError> {
    let todo = db::get_todo_by_id_db(&pool, id).await?;
    Ok(Json(todo))
}
