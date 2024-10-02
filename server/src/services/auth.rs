use anyhow::Result;
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};
use crate::errors::AppError;
use crate::models::{self, auth::Claims};
use crate::errors::user::UserError::InvalidUserNameOrPassword;
use actix_web::{web,HttpRequest};
use crate::global::AppState;
use time::{Duration, OffsetDateTime};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::pkg::dictionary::AUTH_TOKEN;
use rand::Rng;




pub struct Auth;

impl Auth {
    pub async fn login(data: web::Data<AppState>, login: models::Login) -> Result<models::Token, AppError> {
        let user = models::auth::Auth::find_user_by_username(&data.conn, &login).await.map_err(|_|AppError::UserError(InvalidUserNameOrPassword))?;
        let user = user.ok_or(AppError::UserError(InvalidUserNameOrPassword))?;
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        if !Argon2::default().verify_password(login.password.as_bytes(), &parsed_hash).is_ok() {
            return Err(AppError::UserError(InvalidUserNameOrPassword));
        }
        
        let expiration = OffsetDateTime::now_utc() + Duration::seconds(data.config.jwt.expires_time as i64);
        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration.unix_timestamp(),
            iat: OffsetDateTime::now_utc().unix_timestamp(),
            iss: (&data.config.jwt.issuer).to_string(),
        };
        let secret = &data.config.jwt.signing_key.as_bytes(); 
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap();
    

        Ok(models::Token {
            token,
            expire: expiration.unix_timestamp(),
        })
    }
    pub async fn logout(app: web::Data<AppState>, req: HttpRequest) -> Result<(), AppError> {
        if let Some(cookie) = req.cookie(AUTH_TOKEN){
            let token = cookie.value();
            let jwt_expires_time = app.config.jwt.expires_time;
            app.redis_adaptor.add_token_to_black_list(token, jwt_expires_time)?;
        }
        Ok(())
    }

    pub async fn invite(data: web::Data<AppState>, invite: models::Invite) -> Result<(), AppError> {
        let token = generate_token();
        let res = data.redis_adaptor.set_invited_token(token.clone(), invite.email.clone())?;
        data.email_sender.send_register_email(invite.email.clone(),token).map_err(|e| AppError::Error(e.to_string()))?;
        Ok(res)
    }
}


fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    (0..32).map(|_| rng.gen_range(33..127) as u8 as char).collect()
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