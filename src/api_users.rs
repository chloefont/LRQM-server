use crate::models::{User, NewUser, UserTotalDistance, UserTotalTime, PatchUser};
use crate::{AppState};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::{routing::{post, patch, get}, Router};


pub fn stage(app_state: AppState) -> Router {
    Router::new()
        .route("/users", post(user_create).get(users_list))
        .route("/users/:user_id", patch(patch_user).get(get_user))
        .route("/users/:user_id/meters", get(get_user_total_meters))
        .route("/users/:user_id/time", get(get_user_total_time_spent))
        .with_state(app_state)
}

#[utoipa::path(
    post,
    path = "/users",
    description = "Create a user",
    responses(
        (status = 200, description = "User created"),
        (status = 500, description = "Internal server error")
    ),
    request_body = NewUser
)]
pub async fn user_create(State(state): State<AppState>, Json(payload): Json<NewUser>) -> Result<Json<User>, (StatusCode, String)> {
    let user = User::create(
        &state.db.pool,
        payload.username,
        payload.bib_id,
        payload.event_id
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/users/:user_id",
    description = "Get a user by id",
    responses(
        (status = 200, description = "User found"),
        (status = 404, description = "User not found")
    ),
    params(
        ("user_id" = i32, Path, description = "The user id")
    )
)]
pub async fn get_user(State(state): State<AppState>, Path(user_id): Path<i32>) -> Result<Json<User>, (StatusCode, String)> {
    let user = User::get(&state.db.pool, user_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This user does not exist".to_string()))?;

    Ok(Json(user))
}

#[utoipa::path(
    get,
    description = "Get all users",
    path = "/users/",
    responses(
        (status = 200, description = "Users found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn users_list(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = User::all(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(users))
}

#[utoipa::path(
    patch,
    path = "/users/:user_id",
    description = "Edit a user",
    responses(
        (status = 200, description = "User edited"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    request_body = PatchUser
)]
pub async fn patch_user(
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

    let updated_user = User::edit(
        &state.db.pool,
        user
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(updated_user))
}

#[utoipa::path(
    get,
    path = "/users/:user_id/meters",
    description = "Get the total contribution of a user",
    responses(
        (status = 200, description = "Total meters found"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("user_id" = i32, Path, description = "The user id")
    )
)]
pub async fn get_user_total_meters(
    Path(user_id): Path<i32>,
    State(state): State<AppState>
) -> Result<Json<UserTotalDistance>, (StatusCode, String)> {
    let user = User::get(&state.db.pool, user_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This user does not exist".to_string()))?;

    let total_meters: UserTotalDistance = user.get_total_distance(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(total_meters))
}

#[utoipa::path(
    get,
    path = "/users/:user_id/time",
    description = "Get the total time spent by a user",
    responses(
        (status = 200, description = "Total time found"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("user_id" = i32, Path, description = "The user id")
    )
)]
pub async fn get_user_total_time_spent(
    Path(user_id): Path<i32>,
    State(state): State<AppState>
) -> Result<Json<UserTotalTime>, (StatusCode, String)> {
    let user = User::get(&state.db.pool, user_id)
        .await.map_err(|_| (StatusCode::NOT_FOUND, "This user does not exist".to_string()))?;

    let total_time: UserTotalTime = user.get_total_time(&state.db.pool)
        .await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Oh noo !".to_string()))?;

    Ok(Json(total_time))
}