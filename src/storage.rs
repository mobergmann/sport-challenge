use sqlx::sqlite::SqlitePoolOptions;

use crate::hasher;
use crate::user::{BareUser, User};

pub const USER_DB: &str = "sqlite/data.db";

/// Error codes for the storage module
pub enum Error {
    UserNotFound,
    InternalError,
}

pub async fn insert_new_user(user: &BareUser) -> Result<User, Error> {
    let password_hash = hasher::hash(&user.password);

    let pool = SqlitePoolOptions::new().connect(USER_DB).await.unwrap();

    let user: User = sqlx::query_as("insert into users (name, password_hash) values ($1, $2); select * from users where name = $1")
        .bind(&user.name).bind(password_hash)
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok(user)
}

pub async fn get_user(name: &String) -> Result<User, Error> {
    let pool = SqlitePoolOptions::new().connect(USER_DB).await.unwrap();

    let user: User = match sqlx::query_as("select * from users where name = $1")
        .bind(name.as_str())
        .fetch_one(&pool)
        .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            // Return an error if the user does not exist
            return Err(Error::UserNotFound);
        }
        Err(e) => {
            println!("{}", e.to_string());
            // Return an error if something else goes wrong
            return Err(Error::InternalError); // todo return e
        }
    };

    Ok(user)
}

pub async fn user_exists(name: &String) -> bool {
    let user = get_user(name).await;
    match user {
        Ok(_) => true,
        Err(_) => false,
    }
}
