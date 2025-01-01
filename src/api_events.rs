use crate::models::{Event, NewEvent};
use crate::SharedState;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::{routing::{post, get}, Router};
use axum_macros::debug_handler;
use diesel::query_dsl::methods::FindDsl;
use diesel::RunQueryDsl;
use crate::schema::events;

pub fn stage(shared_state: SharedState) -> Router {
    Router::new()
        .route("/events", post(event_create).get(events_list))
        .route("/events/{event_id}/meters", get(event_total_meters))
        .with_state(shared_state)
}

#[debug_handler]
async fn event_create(State(state): State<SharedState>, Json(payload): Json<NewEvent>) -> Result<Json<Event>, (StatusCode, String)> {
    let mut state = state.lock().unwrap();
    let event = Event::create(
        state.db.connection.as_mut().unwrap(),
        payload.name.clone(),
        payload.start_date.clone(),
        payload.end_date.clone(),
        payload.meters_goal.clone()
    ).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(event))
}

async fn events_list(State(state): State<SharedState>) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    let mut state = state.lock().unwrap();
    let events = Event::all(state.db.connection.as_mut().unwrap())
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(events))
}

async fn event_total_meters(State(state): State<SharedState>, Path(event_id): Path<i32>, Json(payload): Json<NewEvent>) -> Result<Json<Event>, (StatusCode, String)> {
    let mut state = state.lock().unwrap();
    let event = events::table.find(event_id).load(state.db.connection.as_mut().unwrap());

    Ok(Json(Event{ id: todo!(), name: todo!(), start_date: todo!(), end_date: todo!(), meters_goal: todo!() }))
}