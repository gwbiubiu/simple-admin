use anyhow::Result;
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};
use crate::{errors::AppError, models};
use sea_orm::*;


pub struct Auth;

impl Auth {
    pub async fn login(db: &DbConn, login: models::Login) -> Result<models::Token, AppError> {
        let user = login.find_user_by_username(db).await?.unwrap();
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        if !Argon2::default().verify_password(login.password.as_bytes(), &parsed_hash).is_ok() {
            return Err(AppError::UserError(crate::errors::user::UserError::InvalidPassword));
        }

        match models::Login::generate_jwt(&user.id.to_string()) {
            Ok(token) => Ok(models::Token {
                token,
                expire: 0, // 设置实际的过期时间
            }),
            Err(_) => Err(AppError::SystemError("生成token失败".to_string())),
        }
    }

    pub async fn logout() {
        // code here
    }
}


#[cfg(test)]
mod tests {
    use argon2::Argon2;
    use password_hash::rand_core::OsRng;
    use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

    #[test]
    fn generate_user_password() {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password("abc123".as_bytes(), &salt).unwrap().to_string();
        println!("password_hash: {}", password_hash);
    }

    #[test]
    fn verify_user_password() {
        let parsed_hash = PasswordHash::new("$argon2id$v=19$m=19456,t=2,p=1$oGAylVBEfogk+BBcTF/R0g$xGKbuFLqFxlhUQMSM8vY6gQVxavbuR/bjYSn3nWyC1o").unwrap();
        assert!(Argon2::default().verify_password("abc123".as_bytes(), &parsed_hash).is_ok());
    }
}