use crate::models::{Event, NewEvent, EventTotalMeters};
use crate::{AppState};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::{routing::{post, get}, Router};
use axum_macros::debug_handler;

pub fn stage(app_state: AppState) -> Router {
    Router::new()
        .route("/events", post(event_create).get(events_list))
        .route("/events/:event_id/meters", get(event_total_meters))
        .with_state(app_state)
}

#[debug_handler]
async fn event_create(State(state): State<AppState>, Json(payload): Json<NewEvent>) -> Result<Json<Event>, (StatusCode, String)> {
    let event = Event::create(
        &state.db.pool,
        payload.name.clone(),
        payload.start_date.clone(),
        payload.end_date.clone(),
        payload.meters_goal.clone()
    ).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(event))
}

async fn events_list(State(state): State<AppState>) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    let events = Event::all(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(events))
}

async fn event_total_meters(State(state): State<AppState>, Path(event_id): Path<i32>) -> Result<Json<EventTotalMeters>, (StatusCode, String)> {
    let event = Event::get(&state.db.pool, event_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This event does not exist".to_string()))?;
    let event_total_meters = event.get_total_distance(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(event_total_meters))
}