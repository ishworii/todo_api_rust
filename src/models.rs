use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodo {
    pub description: String,
}
