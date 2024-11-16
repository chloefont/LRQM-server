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
}