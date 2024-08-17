use std::{collections::HashMap, sync::{Arc, Mutex}};

use axum::Router;
use schemas::User;
mod api_mobile;
mod api_config;
mod schemas;


struct AppState {
    users: HashMap<String, User>
}

type SharedState = Arc<Mutex<AppState>>;


#[tokio::main]
async fn main() {

    let shared_state = Arc::new(Mutex::new(AppState {
        users: HashMap::new()
    }));
    

    let router = Router::new()
        .merge(api_config::stage(shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
