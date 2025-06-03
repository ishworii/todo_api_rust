use crate::errors::AppError;
use crate::models::{CreateTodo, Todo};
use sqlx::SqlitePool;

pub async fn get_all_todos_db(pool: &SqlitePool) -> Result<Vec<Todo>, AppError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id,description,completed FROM todos")
        .fetch_all(pool)
        .await?;
    Ok(todos)
}

pub async fn create_todo_db(pool: &SqlitePool, new_todo: CreateTodo) -> Result<Todo, AppError> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos(id,description,completed) VALUES (NULL,?,?) RETURNING *",
    )
    .bind(new_todo.description)
    .bind(false)
    .fetch_one(pool)
    .await?;
    Ok(todo)
}

pub async fn get_todo_by_id_db(pool: &SqlitePool, id: i64) -> Result<Todo, AppError> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos where id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(todo)
}
