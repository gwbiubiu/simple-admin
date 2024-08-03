use crate::models::user;
use anyhow::Result;
use crate::entities;
use crate::repo::user::IUserRepo;

pub trait IUserService {
    fn create_user(&self, user: user::CreateUser) -> Result<i32>;
    fn get_user_by_id(&self, id: i32) -> Result<user::User>;
}

struct UserService {
    user_repo: Box<dyn IUserRepo>,
}

impl UserService {
    pub fn new(user_repo: Box<dyn IUserRepo>) -> Self {
        UserService { user_repo }
    }
}

impl IUserService for UserService {
    fn create_user(&self, user: user::CreateUser) -> Result<i32> {
        let u = entities::user::User {
            id: 0,
            username: "abc".to_string(),
            email: "abc".to_string(),
            password: "abc".to_string(),
        };
        self.user_repo.create_user(u)?;
        Ok(123)
    }

    fn get_user_by_id(&self, id: i32) -> Result<user::User> {
        todo!()
    }
}
