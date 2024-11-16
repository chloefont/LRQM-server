use axum::{
    routing::post,
    Router
};
use axum::extract::{Json, State};
use crate::models::User;
use crate::{models, SharedState};

pub fn stage(shared_state: SharedState) -> Router {
    
    Router::new()
        .route("/users", post(user_create).get(users_list))
        .with_state(shared_state)
}


async fn user_create(State(state): State<SharedState>, Json(payload): Json<models::User>) {
    println!("{:?}", payload);
    let mut state = state.lock().unwrap();
    // state.users.insert(payload.bib_number.clone(), payload);
}


async fn users_list(State(state): State<SharedState>) -> Json<Vec<User>> {
    let state = state.lock().unwrap();
    // let users: Vec<_> = state.users.values().cloned().collect();
    // Json(users)
    Json(vec![])
}