use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::convert::From;
use std::convert::Into;

#[derive(Clone, Debug, Default, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password_hash: String,
}

impl AuthUser<i64> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}

/// Like user, but instead of storing the password hash, it stores the plain-text password.
/// Is used when the user signs in or signs up. Should not be used outside of these routes.
#[derive(Deserialize)]
pub struct BareUser {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct PublicUser {
    pub id: i64,
    pub name: String,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}
