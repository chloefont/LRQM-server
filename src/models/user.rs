use std::{error::Error, result::Result};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use bigdecimal::BigDecimal;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub bib_id: String,
    pub event_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct UserTotalDistance {
    pub user_id: i32,
    pub meters: i64
}

#[derive(Serialize, Deserialize)]
pub struct UserTotalTime {
    pub user_id: i32,
    pub time: BigDecimal
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub bib_id: String,
    pub event_id: i32
}

impl User {
    pub async fn all(pool: &PgPool) -> Result<Vec<User>, Box<dyn Error>> {
        let result = sqlx::query!(
            "
            SELECT u.id, u.username, u.bib_id, u.event_id
            FROM users u
            "
        ).fetch_all(pool).await?;

        Ok(result
            .into_iter()
            .map(|user| User{
                id: user.id,
                username: user.username,
                bib_id: user.bib_id,
                event_id: user.event_id
            }).collect()
        )
    }

    pub async fn get(pool: &PgPool, user_id: i32) -> Result<User, Box<dyn Error>> {
        let user = sqlx::query!(
            "
            SELECT u.id, u.username, u.bib_id, u.event_id
            FROM users u
            WHERE u.id = $1
            ",
            user_id
        ).fetch_one(pool).await?;

        Ok(User{
            id: user.id,
            username: user.username,
            bib_id: user.bib_id,
            event_id: user.event_id
        })
    }

    pub async fn create(
        pool: &PgPool, 
        username: String, 
        bib_id: String, 
        event_id: i32
    ) -> Result<User, Box<dyn Error>> {
        let user = sqlx::query!(
            "
            INSERT INTO users (username, bib_id, event_id)
            VALUES ($1, $2, $3)
            RETURNING id, username, bib_id, event_id
            ",
            username,
            bib_id,
            event_id
        ).fetch_one(pool).await?;

        Ok(User{
            id: user.id,
            username: user.username,
            bib_id: user.bib_id,
            event_id: user.event_id
        })
    }

    pub async fn edit(
        pool: &PgPool,
        user: User
    ) -> Result<User, Box<dyn Error>> {
        let user = sqlx::query!(
            "
            UPDATE users
            SET username = $1, bib_id = $2, event_id = $3
            WHERE id = $4
            RETURNING id, username, bib_id, event_id
            ",
            user.username,
            user.bib_id,
            user.event_id,
            user.id
        ).fetch_one(pool).await?;

        Ok(User{
            id: user.id,
            username: user.username,
            bib_id: user.bib_id,
            event_id: user.event_id
        })
    }

    pub async fn get_total_distance(
        self,
        pool: &PgPool
    ) -> Result<UserTotalDistance, Box<dyn Error>> {
        let result = sqlx::query!(
            "
            SELECT SUM(meters) as total_meters
            FROM measures
            WHERE user_id = $1
            ",
            self.id
        ).fetch_one(pool).await?;

        Ok(UserTotalDistance{
            user_id: self.id,
            meters: result.total_meters.unwrap_or(0)
        })
    }

    pub async fn get_total_time(
        self,
        pool: &PgPool
    ) -> Result<UserTotalTime, Box<dyn Error>> {
        let result = sqlx::query!(
            "
            SELECT SUM(EXTRACT(EPOCH FROM end_time - start_time)) as total_time
            FROM measures
            WHERE user_id = $1
            ",
            self.id
        ).fetch_one(pool).await?;

        Ok(UserTotalTime{
            user_id: self.id,
            time: result.total_time.unwrap_or(BigDecimal::from(0))
        })
    }
}