
use std::error::Error;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;


#[derive(Serialize, Deserialize)]
pub struct Measure {
    pub id: i32,
    pub user_id: i32,
    pub meters: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize)]
pub struct NewMeasure {
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub meters: i32,
    pub end_time: Option<NaiveDateTime>
}

impl Measure {
    pub async fn create(pool: &PgPool, user_id: i32, meters: i32, start_time: NaiveDateTime, end_time: Option<NaiveDateTime>) -> Result<Measure, Box<dyn Error>> {
        let result = sqlx::query!(
            "
            INSERT INTO measures (user_id, meters, start_time, end_time)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, meters, start_time, end_time
            ",
            user_id,
            meters,
            start_time,
            end_time
        ).fetch_one(pool).await?;

        Ok(Measure{
            id: result.id,
            user_id: result.user_id,
            meters: result.meters,
            start_time: result.start_time,
            end_time: result.end_time
        })
    }

    pub async fn edit(pool: &PgPool, measure: Measure) -> Result<Measure, Box<dyn Error>> {
        let result = sqlx::query!(
            "
            INSERT INTO measures (user_id, meters, start_time, end_time)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, meters, start_time, end_time
            ",
            measure.user_id,
            measure.meters,
            measure.start_time,
            measure.end_time
        ).fetch_one(pool).await?;

        Ok(Measure{
            id: result.id,
            user_id: result.user_id,
            meters: result.meters,
            start_time: result.start_time,
            end_time: result.end_time
        })
    }
}