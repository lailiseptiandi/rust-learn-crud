use crate::models::user::{User, UserResponse};
use crate::repository::user_repository::UserRepository;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        UserService { repo }
    }

    pub fn get_all_users(&self) -> Result<Vec<UserResponse>, mysql::Error> {
        self.repo.get_all_users()
    }

    pub fn create_user(&self, user: User) -> Result<(), mysql::Error> {
        self.repo.create_user(user)
    }
}
