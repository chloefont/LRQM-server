use std::error::Error;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;


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

    pub async fn get(pool: &PgPool, user_id: i32) -> Result<User, Box<dyn Error>> {
        let user = sqlx::query!(
            "
            SELECT u.id, u.username, u.bib_id, u.event_id, u.total_meters
            FROM users u
            WHERE u.id = $1
            ",
            user_id
        ).fetch_one(pool).await?;

        Ok(User{
            id: user.id,
            username: user.username,
            bib_id: user.bib_id,
            event_id: user.event_id,
            total_meters: user.total_meters
        })
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

    pub async fn edit(
        pool: &PgPool,
        user: User
    ) -> Result<User, Box<dyn Error>> {
        let user = sqlx::query!(
            "
            UPDATE users
            SET username = $1, bib_id = $2, event_id = $3, total_meters = $4
            WHERE id = $5
            RETURNING id, username, bib_id, event_id, total_meters
            ",
            user.username,
            user.bib_id,
            user.event_id,
            user.total_meters,
            user.id
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