use std::os::macos::raw::stat;

use crate::models::{User, NewUser, Event, NewEvent};
use crate::{AppState, SharedState};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::{routing::post, Router};

pub fn stage(app_state: AppState) -> Router {
    Router::new()
        .route("/users", post(user_create).get(users_list))
        .with_state(app_state)
}

async fn user_create(State(state): State<AppState>, Json(payload): Json<NewUser>) -> Result<Json<User>, (StatusCode, String)> {
    let user = User::create(
        &state.db.pool,
        payload.username,
        payload.bib_id,
        payload.event_id,
        payload.total_meters
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

async fn users_list(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = User::all(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(users))
}