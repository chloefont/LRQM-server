use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::users;
use diesel::{result::Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub bib_id: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub bib_id: String
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

    pub fn create(conn: &mut PgConnection, username: String, bib_id: String) -> Result<User, Error> {
        let new_user = NewUser {
            username,
            bib_id
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
    }
}