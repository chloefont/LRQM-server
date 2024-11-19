use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::users;
use diesel::{result::Error};
use crate::models::event::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Associations)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Event))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub bib_id: String,
    pub event_id: i32,
    pub total_meters: i32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub bib_id: String,
    pub event_id: i32,
    pub total_meters: Option<i32>
}

impl User {
    pub fn all(conn: &mut PgConnection) -> Result<Vec<User>, Error> {
        // users::table.load::<User>(&*connection).expect("Error loading users")
        let q_result = users::table.load(conn);

        match q_result {
            Ok(result) => {
                Ok(result)
            },
            Err(e) => {
                Err(Error::from(e))
            }
        }
    }

    pub fn create(
        conn: &mut PgConnection, 
        username: String, 
        bib_id: String, 
        event: Event,
        total_meters: Option<i32>
    ) -> Result<User, Error> {
        let new_user = NewUser {
            total_meters: total_meters,
            username: username,
            bib_id: bib_id,
            event_id: event.id
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
    }
}