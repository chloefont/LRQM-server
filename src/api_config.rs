use axum::{
    routing::post,
    Router
};
use axum::extract::{Json, State};
use crate::{schemas, SharedState};

pub fn stage(shared_state: SharedState) -> Router {
    
    Router::new()
        .route("/users", post(user_create)).with_state(shared_state)
}


async fn user_create(State(state): State<SharedState>, Json(payload): Json<schemas::User>) {
    println!("{:?}", payload);
    let mut state = state.lock().unwrap();
    state.users.insert(payload.bib_number.clone(), payload);
}