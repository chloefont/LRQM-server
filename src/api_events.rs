use crate::models::{Event, EventActiveUsersNumber, EventTotalMeters, NewEvent};
use crate::{AppState};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::{routing::{post, get}, Router};

pub fn stage(app_state: AppState) -> Router {
    Router::new()
        .route("/events", post(event_create).get(events_list))
        .route("/events/:event_id", get(get_event))
        .route("/events/:event_id/meters", get(event_total_meters))
        .route("/events/:event_id/active_users", get(get_event_active_users_number))
        .with_state(app_state)
}

#[utoipa::path(
    post,
    path = "/events",
    description = "Create an event",
    responses(
        (status = 200, description = "Event created"),
        (status = 500, description = "Internal server error")
    ),
    request_body = NewEvent
)]
pub async fn event_create(State(state): State<AppState>, Json(payload): Json<NewEvent>) -> Result<Json<Event>, (StatusCode, String)> {
    let event = Event::create(
        &state.db.pool,
        payload.name.clone(),
        payload.start_date.clone(),
        payload.end_date.clone(),
        payload.meters_goal.clone()
    ).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(event))
}

#[utoipa::path(
    get,
    path = "/events/:event_id",
    description = "Get an event by id",
    responses(
        (status = 200, description = "Event found"),
        (status = 404, description = "Event not found")
    ),
    params(
        ("event_id" = i32, Path, description = "The event id")
    )
)]
pub async fn get_event(State(state): State<AppState>, Path(event_id): Path<i32>) -> Result<Json<Event>, (StatusCode, String)> {
    let event = Event::get(&state.db.pool, event_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This event does not exist".to_string()))?;

    Ok(Json(event))
}

#[utoipa::path(
    get,
    description = "Get all events",
    path = "/events/",
    responses(
        (status = 200, description = "Events found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn events_list(State(state): State<AppState>) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    let events = Event::all(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(events))
}

#[utoipa::path(
    get,
    path = "/events/:event_id/meters",
    description = "Get the total meters of an event",
    responses(
        (status = 200, description = "Total meters found"),
        (status = 404, description = "Event not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("event_id" = i32, Path, description = "The event id")
    )
)]
pub async fn event_total_meters(State(state): State<AppState>, Path(event_id): Path<i32>) -> Result<Json<EventTotalMeters>, (StatusCode, String)> {
    let event = Event::get(&state.db.pool, event_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This event does not exist".to_string()))?;
    let event_total_meters = event.get_total_distance(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(event_total_meters))
}


#[utoipa::path(
    get,
    path = "/events/:event_id/active_users",
    description = "Get the number of active users of an event",
    responses(
        (status = 200, description = "Active users number found"),
        (status = 404, description = "Event not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("event_id" = i32, Path, description = "The event id")
    )
)]
pub async fn get_event_active_users_number(State(state): State<AppState>, Path(event_id): Path<i32>) -> Result<Json<EventActiveUsersNumber>, (StatusCode, String)> {
    let event = Event::get(&state.db.pool, event_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This event does not exist".to_string()))?;
    let active_users_number = event.get_active_users_number(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(active_users_number))
}