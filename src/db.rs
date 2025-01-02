use dotenvy::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};


#[derive(Clone)]
pub struct PostgresDb {
    pub pool: Pool<Postgres>
}


impl PostgresDb {
    pub async fn new() -> Result<Self, Error> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Ok(PostgresDb{pool: PgPoolOptions::new().max_connections(5).connect(&database_url).await? })
    }
}