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
pub struct EventTotalMeters {
    pub event_id: i32,
    pub total_meters: i64
}

#[derive(Serialize, Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub meters_goal: i32
}

impl Event {
    pub async fn get(pool: &PgPool, id: i32) -> Result<Event, Box<dyn Error>> {
        let event = sqlx::query!(
            "
            SELECT e.id, e.name, e.start_date, e.end_date, e.meters_goal
            FROM events e
            WHERE e.id = $1
            ",
            id
        ).fetch_one(pool).await?;

        Ok(Event{
            id: event.id,
            name: event.name,
            start_date: event.start_date,
            end_date: event.end_date,
            meters_goal: event.meters_goal
        })
    }

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

    pub async fn get_total_distance(self, pool: &PgPool) -> Result<EventTotalMeters, Box<dyn Error>> {
        let event_total_meters = sqlx::query!(
            "
            SELECT SUM(m.meters) as sum
            FROM users u
            INNER JOIN events e 
                ON e.id = u.event_id
                AND e.id = $1
            INNER JOIN measures m
                ON m.user_id = u.id
            ",
            self.id
        ).fetch_one(pool).await?;

        Ok(EventTotalMeters{
            event_id: self.id,
            total_meters: event_total_meters.sum.unwrap_or(0)
        })
    }
}