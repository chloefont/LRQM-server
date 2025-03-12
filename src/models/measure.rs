
use std::error::Error;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct Measure {
    pub id: i32,
    pub user_id: i32,
    pub contributors_number: i32,
    pub meters: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewMeasure {
    pub user_id: i32,
    pub contributors_number: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EditMeters {
    pub meters: i32
}

impl Measure {
    pub async fn create(
        pool: &PgPool, 
        user_id: i32,
        contributors_number: Option<i32>,
        meters: i32, 
        start_time: NaiveDateTime, 
        end_time: Option<NaiveDateTime>
    ) -> Result<Measure, Box<dyn Error>> {
        let result = sqlx::query!(
            "
            INSERT INTO measures (user_id, contributors_number, meters, start_time, end_time)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, contributors_number, meters, start_time, end_time
            ",
            user_id,
            contributors_number,
            meters,
            start_time,
            end_time
        ).fetch_one(pool).await?;

        Ok(Measure{
            id: result.id,
            user_id: result.user_id,
            contributors_number: result.contributors_number,
            meters: result.meters,
            start_time: result.start_time,
            end_time: result.end_time
        })
    }

    pub async fn get(
        pool: &PgPool,
        id: i32
    ) -> Result<Measure, Box<dyn Error>> {
        let result = sqlx::query!(
            "
            SELECT id, user_id, contributors_number, meters, start_time, end_time
            FROM measures
            WHERE id = $1
            ",
            id
        ).fetch_one(pool).await?;

        Ok(Measure{
            id: result.id,
            user_id: result.user_id,
            contributors_number: result.contributors_number,
            meters: result.meters,
            start_time: result.start_time,
            end_time: result.end_time
        })
    }

    pub async fn edit(pool: &PgPool, measure: Measure) -> Result<Measure, Box<dyn Error>> {
        let result = sqlx::query!(
            "
            INSERT INTO measures (user_id, contributors_number, meters, start_time, end_time)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, contributors_number, meters, start_time, end_time
            ",
            measure.user_id,
            measure.contributors_number,
            measure.meters,
            measure.start_time,
            measure.end_time
        ).fetch_one(pool).await?;

        Ok(Measure{
            id: result.id,
            user_id: result.user_id,
            contributors_number: result.contributors_number,
            meters: result.meters,
            start_time: result.start_time,
            end_time: result.end_time
        })
    }
}