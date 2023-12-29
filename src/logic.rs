pub mod account;
pub mod activities;
pub mod auth;
pub mod users;

use crate::account::Account;
use axum_login::SqliteStore;

type AuthContext = axum_login::extractors::AuthContext<i64, Account, SqliteStore<Account>>;
