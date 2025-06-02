use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodo {
    pub description: String,
}
