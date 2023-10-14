use sqlx::sqlite::SqlitePoolOptions;
use crate::activity::{Activity, BareActivity};

use crate::hasher;
use crate::user::{BareUser, User};

pub const DB_URI: &str = "sqlite/data.db";

/// Error codes for the storage module
pub enum Error {
    ElementNotFound,
    InternalError,
    NotImplemented
}

pub async fn insert_new_user(user: &BareUser) -> Result<User, Error> {
    let password_hash = hasher::hash(&user.password);

    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();

    let user: User = sqlx::query_as("insert into users (name, password_hash) values ($1, $2) returning *")
        .bind(&user.name).bind(password_hash)
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok(user)
}

pub async fn get_user(name: &String) -> Result<User, Error> {
    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();

    let user: User = match sqlx::query_as("select * from users where name = $1")
        .bind(name.as_str())
        .fetch_one(&pool)
        .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => return Err(Error::ElementNotFound),
        Err(_) => return Err(Error::InternalError), // todo return e
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


pub async fn get_activity(id: i64) -> Result<Activity, Error> {
    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();

    let activity: Activity = match sqlx::query_as("select * from activities where id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(activity) => activity,
        Err(sqlx::Error::RowNotFound) => return Err(Error::ElementNotFound),
        Err(e) => {
            return Err(Error::InternalError);
        }, // todo return e
    };

    Ok(activity)
}

pub async fn get_all_activities() -> Result<Vec<Activity>, Error> {
    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();

    let activities: Vec<Activity> = match sqlx::query_as("select * from activities")
        .fetch_all(&pool)
        .await
    {
        Ok(activities) => activities,
        Err(sqlx::Error::RowNotFound) => return Err(Error::ElementNotFound),
        Err(_) => return Err(Error::InternalError), // todo return e
    };

    Ok(activities)
}

pub async fn new_activity(activity: &BareActivity, author: &User) -> Result<Activity, Error> {
    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();

    let activity: Activity = sqlx::query_as("insert into activities (author_id, amount, activity_type, start_time, end_time) values ($1, $2, $3, $4, $5) returning *")
        .bind(author.id)
        .bind(activity.amount)
        .bind(&activity.activity_type)
        .bind(activity.start_time)
        .bind(activity.end_time)
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok(activity)
}

pub async fn edit_activity(activity: &BareActivity) -> Result<Activity, Error> {
    Err(Error::NotImplemented)
}

pub async fn delete_activity(id: i64) -> Result<Activity, Error> {
    let pool = SqlitePoolOptions::new().connect(DB_URI).await.unwrap();

    let activity: Activity = sqlx::query_as("delete from activities where id = $1 returning *")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok(activity)
}
