use crate::models::user::{User, UserResponse};
use mysql::prelude::*;
use mysql::Pool;
extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};

pub struct UserRepository {
    pool: Pool,
}

impl UserRepository {
    pub fn new(pool: Pool) -> Self {
        UserRepository { pool }
    }

    pub fn get_all_users(&self) -> Result<Vec<UserResponse>, mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        let users: Vec<User> = conn.query_map(
            "SELECT id, name, email, password FROM users",
            |(id, name, email, password)| User {
                id,
                name,
                email,
                password,
            },
        )?;

        let users_response: Vec<UserResponse> = users
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
            })
            .collect();

        Ok(users_response)
    }

    pub fn create_user(&self, user: User) -> Result<(), mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        let password = user.password.clone().unwrap_or_else(|| "".to_string());
        let pass_hash = hash(password, DEFAULT_COST).unwrap();
        conn.exec_drop(
            "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
            (user.name.clone(), user.email.clone(), pass_hash),
        )?;
        Ok(())
    }
}
