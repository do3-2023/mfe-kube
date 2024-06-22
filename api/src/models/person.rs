use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct Person {
    pub id: i32,
    pub last_name: String,
    pub phone_number: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePerson {
    #[validate(length(max = 2000))]
    pub last_name: String,
    pub phone_number: String,
}
