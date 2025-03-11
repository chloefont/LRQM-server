use crate::models::{EditMeters, Event, Measure, NewMeasure, User};
use axum::{extract::{Path, State}, http::StatusCode, routing::{post, put}, Json, Router};
use chrono::Utc;


use crate::AppState;

pub fn stage(app_state: AppState) -> Router {
    Router::new()
        .route("/measures/start", post(start_measuring))
        .route("/measures/:measure_id", put(edit_meters))
        .route("/measures/:measure_id/stop", put(stop_meters))
        .with_state(app_state)
}

async fn start_measuring(State(state): State<AppState>, Json(payload): Json<NewMeasure>) -> Result<Json<Measure>, (StatusCode, String)> {
    let user = User::get(&state.db.pool, payload.user_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This user does not exist".to_string()))?;

    let event = Event::get(&state.db.pool, user.event_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This event does not exist".to_string()))?;

    let start_date = Utc::now().naive_utc();

    // Check measure has not started before event start date
    if start_date < event.start_date {
        return Err((StatusCode::BAD_REQUEST, "The event has not started yet".to_string()));
    } 

    // Check measure has not ended after event end date
    if start_date > event.end_date {
        return Err((StatusCode::BAD_REQUEST, "The event has already ended".to_string()));
    }

    let measure = Measure::create(
        &state.db.pool,
        payload.user_id,
        payload.contributors_number,
        0,
        Utc::now().naive_utc(),
        Option::None
    ).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(measure))
}



async fn edit_meters(State(state): State<AppState>,  Path(measure_id): Path<i32>, Json(payload): Json<EditMeters>) -> Result<Json<Measure>, (StatusCode, String)> {
    let measure = Measure::get(&state.db.pool, measure_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This measure does not exist".to_string()))?;

    if payload.meters < 0 {
        return Err((StatusCode::BAD_REQUEST, "Meters must be positive".to_string()));
    }

    let measure = Measure::edit(
        &state.db.pool,
        Measure{
            id: measure_id,
            user_id: measure.user_id,
            contributors_number: measure.contributors_number,
            meters: payload.meters,
            start_time: measure.start_time,
            end_time: measure.end_time
        }
    ).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(measure))
}

async fn stop_meters(State(state): State<AppState>, Path(measure_id): Path<i32>) -> Result<Json<Measure>, (StatusCode, String)> {
    let measure = Measure::get(&state.db.pool, measure_id)
    .await.map_err(|_| (StatusCode::NOT_FOUND, "This measure does not exist".to_string()))?;

    let user = User::get(&state.db.pool, measure.user_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This user does not exist".to_string()))?;

    let event = Event::get(&state.db.pool, user.event_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This event does not exist".to_string()))?;

   let end_date = Utc::now().naive_utc();

   // Check measure has not ended before measure start date
    if end_date < event.start_date {
         return Err((StatusCode::BAD_REQUEST, "The measure has not started yet".to_string()));
    }

    // Check measure has not ended after measure end date
    if end_date > event.end_date {
        return Err((StatusCode::BAD_REQUEST, "The measure has already ended".to_string()));
    }

    let measure = Measure::edit(
        &state.db.pool,
        Measure{
            id: measure.id,
            user_id: measure.user_id,
            contributors_number: measure.contributors_number,
            meters: measure.meters,
            start_time: measure.start_time,
            end_time: Option::Some(end_date)
        }
    ).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(measure))
}