use std::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{NaiveDateTime};


#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub meters_goal: i32
}

#[derive(Serialize, Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub meters_goal: i32
}

impl Event {
    pub async fn create(pool: &PgPool, name: String, start_date: NaiveDateTime, end_date: NaiveDateTime, meters_goal: i32) -> Result<Event, Box<dyn Error>> {
        let event = sqlx::query!(
            "
            INSERT INTO events (name, start_date, end_date, meters_goal)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, start_date, end_date, meters_goal
            ",
            name,
            start_date,
            end_date,
            meters_goal
        ).fetch_one(pool).await?;

        Ok(Event{
            id: event.id,
            name: event.name,
            start_date: event.start_date,
            end_date: event.end_date,
            meters_goal: event.meters_goal
        })
    }

    pub async fn all(pool: &PgPool) -> Result<Vec<Event>, Box<dyn Error>> {
        let result = sqlx::query!(
            "SELECT e.id, e.name, e.start_date, e.end_date, e.meters_goal
            FROM events e
            "
        ).fetch_all(pool).await?;

        Ok(result
            .into_iter()
            .map(
                |event| Event{
                    id: event.id,
                    name: event.name,
                    start_date: event.start_date,
                    end_date: event.end_date,
                    meters_goal: event.meters_goal
                }
            ).collect()
        )
    }

    pub async fn total_meters_for_event(pool: &PgPool, id: i32) -> Result<i64, Box<dyn Error>> {
        // _ = events::table.find(id).first::<Event>(conn).map_err(|e| Error::from(e));

        // let q_result = users::table
        //     .filter(users::event_id.eq(id))
        //     .select(diesel::dsl::sum(users::total_meters))
        //     .first::<Option<i64>>(conn)
        //     .map(|opt| opt.unwrap_or(0));

        // match q_result {
        //     Ok(result) => {
        //         Ok(result)
        //     },
        //     Err(e) => {
        //         Err(Error::from(e))
        //     }
        // }

        let event_total_meters = sqlx::query!(
            "
            SELECT SUM(u.total_meters)
            FROM users u
            INNER JOIN events e ON e.id = u.event_id
            WHERE e.id = $1
            ",
            id
        ).fetch_one(pool).await?;

        Ok(event_total_meters.sum.unwrap_or_default())
    }
}