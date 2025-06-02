mod db;
mod errors;
mod handlers;
mod models;
mod routes;

use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[tokio::main]
async fn main(){
    dotenvy::dotenv().expect("Failed to read .env file");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    println!("Migations complete,pool created");

    let app = routes::create_router(pool);
    
    let url = "127.0.0.1:8000";
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    println!("Server running on {}",url);
    axum::serve(listener,app).await.unwrap();
}
