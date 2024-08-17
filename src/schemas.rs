use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub bib_number: String
}


