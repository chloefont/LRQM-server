use postgres::{Client, NoTls};
use diesel::{pg::Pg, prelude::*, result::Error};
use dotenvy::dotenv;
use std::env;
use std::f32::consts::E;
use crate::schema::users;
use crate::models::User;

pub struct PostgresDb {
    pub connection: Option<PgConnection>
}

pub trait DbApi {
    fn connect(&mut self);
    fn put_users(&self);
    fn get_users(&mut self) -> Result<Vec<User>, Error>;
}

impl DbApi for PostgresDb {
    fn connect(&mut self) {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        self.connection = Some(PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)));
    }

    fn put_users(&self) {
        print!("put_users");
    }

    fn get_users(&mut self) -> Result<Vec<User>, Error> {
        let connection = self.connection.as_mut().unwrap();
        // users::table.load::<User>(&*connection).expect("Error loading users")
        let q_result = users::table.load(connection);

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