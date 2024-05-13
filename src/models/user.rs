use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]

pub struct User {
    pub id: Option<u64>,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Option<u64>,
    pub name: String,
    pub email: String,
}
