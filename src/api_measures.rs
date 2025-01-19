use crate::models::{Measure, NewMeasure, User, Event};
use axum::{extract::State, http::StatusCode, routing::{post, put}, Json, Router};


use crate::AppState;

pub fn stage(app_state: AppState) -> Router {
    Router::new()
        .route("/measures", post(start_measuring))
        .route("/measures", put(edit(edit_measure)))
        .with_state(app_state)
}

async fn start_measuring(State(state): State<AppState>, Json(payload): Json<NewMeasure>) -> Result<Json<Measure>, (StatusCode, String)> {
    let user = User::get(&state.db.pool, payload.user_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This user does not exist".to_string()))?;

    let event = Event::get(&state.db.pool, user.event_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This event does not exist".to_string()))?;

    if event.start_date > payload.start_time || event.end_date < payload.start_time {
        return Err((StatusCode::BAD_REQUEST, "The event is not happening at this time".to_string()));
    }

    let measure = Measure::create(
        &state.db.pool,
        payload.user_id,
        payload.meters,
        payload.start_time,
        payload.end_time
    ).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(measure))
}

async fn edit_measure(State(state): State<AppState>, Json(payload): Json<Measure>) -> Result<Json<Measure>, (StatusCode, String)> {
    let user: User = User::get(&state.db.pool, payload.user_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This user does not exist".to_string()))?;

    let event = Event::get(&state.db.pool, user.event_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This event does not exist".to_string()))?;

    if event.start_date > payload.start_time || event.end_date < payload.start_time {
        return Err((StatusCode::BAD_REQUEST, "The event is not happening at this time".to_string()));
    }

    let measure = Measure::edit(
        &state.db.pool,
        Measure{
            id: payload.id,
            user_id: payload.user_id,
            meters: payload.meters,
            start_time: payload.start_time,
            end_time: payload.end_time
        }
    ).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(measure))
}