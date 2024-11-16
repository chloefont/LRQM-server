use postgres::{Client, NoTls};
use diesel::{pg::Pg, prelude::*, result::Error};
use dotenvy::dotenv;
use std::env;
use std::f32::consts::E;
use crate::schema::users;

pub struct PostgresDb {
    pub connection: Option<PgConnection>
}

pub trait DbApi {
    fn connect(&mut self);
}

impl DbApi for PostgresDb {
    fn connect(&mut self) {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        self.connection = Some(PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)));
    }
}