use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Person {
    pub id: i32,
    pub last_name: String,
    pub phone_number: String,
    pub location: String,
}
