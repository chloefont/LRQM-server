use crate::models::{User, NewUser, Event, NewEvent};
use crate::schema::events;
use crate::SharedState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::{routing::post, Router};
use diesel::{QueryDsl, RunQueryDsl};

pub fn stage(shared_state: SharedState) -> Router {
    Router::new()
        .route("/users", post(user_create).get(users_list))
        .with_state(shared_state)
}

async fn user_create(State(state): State<SharedState>, Json(payload): Json<NewUser>) -> Result<Json<User>, (StatusCode, String)> {
    let mut state = state.lock().unwrap();

    let related_event: Event = events::table.find(payload.event_id).first(state.db.connection.as_mut().unwrap())
        .map_err(|_| (StatusCode::NOT_FOUND, "Event not found".to_string()))?;

    let user = User::create(
        state.db.connection.as_mut().unwrap(),
        payload.username.clone(),
        payload.bib_id.clone(),
        related_event,
        payload.total_meters.clone()
    ).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(user))
}

async fn users_list(
    State(state): State<SharedState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut state = state.lock().unwrap();
    let users = User::all(state.db.connection.as_mut().unwrap())
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(users))
}