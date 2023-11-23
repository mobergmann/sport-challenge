use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::convert::From;
use std::convert::Into;

/// Object storing all Account related data
#[derive(Clone, Debug, Default, Deserialize, Serialize, FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub password_hash: String,
}

impl AuthUser<i64> for Account {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}

/// A subset of the `Account` object, but only contains the properties which can be edited directly.
#[derive(Deserialize)]
pub struct EditAccount {
    pub name: String
}

/// Like `Account`, but instead of storing the password hash, it stores the plain-text password.
/// Is used when the user logs-in or signs-up. !!Should not be used outside of these routes!!
#[derive(Deserialize)]
pub struct BareAccount {
    pub name: String,
    pub password: String,
}

/// `User` is like `Account` but does not contain sensible information,
/// like the accounts password-hash or the E-Mail
#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

/// Because a `User` is a subset of the `Account` we can convert any `Account` into a `User`.
impl From<Account> for User {
    fn from(user: Account) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}
