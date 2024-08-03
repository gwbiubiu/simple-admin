use crate::entities::user::User;
use anyhow::Result;

pub trait IUserRepo {
    fn create_user(&self, user: User) -> Result<()>;
    fn get_user_by_id(&self, id: i32) -> Result<User>;
}

struct UserRepo {
    pool: mysql::Pool,
}

impl UserRepo {
    pub fn new(pool: mysql::Pool) -> Self {
        UserRepo { pool }
    }
}

impl IUserRepo for UserRepo {
    fn create_user(&self, user: User) -> Result<()> {
        Ok(())
    }

    fn get_user_by_id(&self, id: i32) -> Result<User> {
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        };
        Ok(user)
    }
}
