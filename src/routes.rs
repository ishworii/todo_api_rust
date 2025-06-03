use crate::handlers;
use axum::{routing::get,Router};
use sqlx::SqlitePool;

pub fn create_router(db_pool : SqlitePool) -> Router{
    Router::new()
        .route("/todos",get(handlers::get_todos).post(handlers::create_todo))
        .route("/todos/{id}",get(handlers::get_todo_by_id))
        .with_state(db_pool)
}
