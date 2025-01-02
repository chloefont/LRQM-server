use std::error::Error;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::event::Event;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub bib_id: String,
    pub event_id: i32,
    pub total_meters: i32
}


#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub bib_id: String,
    pub event_id: i32,
    pub total_meters: Option<i32>
}

impl User {
    pub async fn all(pool: &PgPool) -> Result<Vec<User>, Box<dyn Error>> {
        // users::table.load::<User>(&*connection).expect("Error loading users")
        let result = sqlx::query!(
            "
            SELECT u.id, u.username, u.bib_id, u.event_id, u.total_meters
            FROM users u
            "
        ).fetch_all(pool).await?;

        Ok(result
            .into_iter()
            .map(|user| User{
                id: user.id,
                username: user.username,
                bib_id: user.bib_id,
                event_id: user.event_id,
                total_meters: user.total_meters
            }).collect()
        )
    }

    pub async fn create(
        pool: &PgPool, 
        username: String, 
        bib_id: String, 
        event_id: i32,
        total_meters: Option<i32>
    ) -> Result<User, Box<dyn Error>> {
        let user = sqlx::query!(
            "
            INSERT INTO users (username, bib_id, event_id, total_meters)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, bib_id, event_id, total_meters
            ",
            username,
            bib_id,
            event_id,
            total_meters.unwrap_or_default()
        ).fetch_one(pool).await?;

        Ok(User{
            id: user.id,
            username: user.username,
            bib_id: user.bib_id,
            event_id: user.event_id,
            total_meters: user.total_meters
        })
    }
}