use axum::Router;
use db::{PostgresDb};
use utoipa::{OpenApi};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
mod api_users;
mod api_events;
mod api_measures;
mod models;
mod db;

#[derive(Clone)]
struct AppState {
    pub db: db::PostgresDb,
}

#[derive(OpenApi)]
#[openapi(paths(
    api_events::event_create,
    api_events::get_event,
    api_events::events_list,
    api_events::event_total_meters,
    api_events::get_event_active_users_number,
    api_measures::start_measuring,
    api_measures::edit_meters,
    api_measures::stop_meters,
    api_users::user_create,
    api_users::get_user,
    api_users::patch_user,
    api_users::users_list,
    api_users::get_user_total_meters,
    api_users::get_user_total_time_spent
))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    dotenv().ok();
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app_state = AppState {
        db : PostgresDb::new().await.expect("Could not connect to db"),
    };

    let router = Router::new()
        .merge(api_events::stage(app_state.clone()))
        .merge(api_users::stage(app_state.clone()))
        .merge(api_measures::stage(app_state.clone()))
        .layer(TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
            tracing::info_span!(
                "request",
                method = %request.method(),
                uri = %request.uri(),
                headers = ?request.headers(),
            )
        }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
