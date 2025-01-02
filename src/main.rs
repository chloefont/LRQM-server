use std::{sync::{Arc, Mutex}, thread};

use axum::Router;
use db::{PostgresDb};
use dotenvy::dotenv;
mod api_users;
mod api_events;
mod models;
mod db;


#[derive(Clone)]
struct AppState {
    pub db: db::PostgresDb,
}

type SharedState = Arc<Mutex<AppState>>;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state = AppState {
        db : PostgresDb::new().await.expect("Could not connect to db"),
    };

    let router = Router::new()
        .merge(api_events::stage(app_state.clone()))
        .merge(api_users::stage(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
