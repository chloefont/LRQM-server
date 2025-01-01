use serde::{Deserialize, Serialize};
use diesel::{prelude::*};
use crate::schema::{events, users};
use diesel::{result::Error};
use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub meters_goal: i32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::events)]
pub struct NewEvent {
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub meters_goal: i32
}

impl Event {
    pub fn create(conn: &mut PgConnection, name: String, start_date: NaiveDateTime, end_date: NaiveDateTime, meters_goal: i32) -> Result<Event, Error> {
        let new_event = NewEvent {
            name,
            start_date,
            end_date,
            meters_goal
        };

        diesel::insert_into(events::table)
            .values(&new_event)
            .returning(Event::as_returning())
            .get_result(conn)
    }

    pub fn all(conn: &mut PgConnection) -> Result<Vec<Event>, Error> {
        let q_result = events::table.load(conn);

        match q_result {
            Ok(result) => {
                Ok(result)
            },
            Err(e) => {
                Err(Error::from(e))
            }
        }
    }

    pub fn total_meters_for_event(conn: &mut PgConnection, id: i32) -> Result<i64, Error> {
        _ = events::table.find(id).first::<Event>(conn).map_err(|e| Error::from(e));

        let q_result = users::table
            .filter(users::event_id.eq(id))
            .select(diesel::dsl::sum(users::total_meters))
            .first::<Option<i64>>(conn)
            .map(|opt| opt.unwrap_or(0));

        match q_result {
            Ok(result) => {
                Ok(result)
            },
            Err(e) => {
                Err(Error::from(e))
            }
        }
    }
}