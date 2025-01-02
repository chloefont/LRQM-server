use crate::models::{User, NewUser, Event, NewEvent};
use crate::{AppState};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::{routing::{post, patch}, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PatchUser {
    pub username: Option<String>,
    pub bib_id: Option<String>,
    pub event_id: Option<i32>,
    pub total_meters: Option<i32>
}

pub fn stage(app_state: AppState) -> Router {
    Router::new()
        .route("/users", post(user_create).get(users_list))
        .route("/users/:user_id", patch(patch_user))
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

async fn patch_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<PatchUser>
) -> Result<Json<User>, (StatusCode, String)> {
    let mut user = User::get(&state.db.pool, user_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This user does not exist".to_string()))?;

    if let Some(username) = payload.username {
        user.username = username;
    }

    if let Some(bib_id) = payload.bib_id {
        user.bib_id = bib_id;
    }

    if let Some(event_id) = payload.event_id {
        user.event_id = event_id;
    }

    if let Some(total_meters) = payload.total_meters {
        user.total_meters = total_meters;
    }

    let updated_user = User::edit(
        &state.db.pool,
        user
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(updated_user))
}