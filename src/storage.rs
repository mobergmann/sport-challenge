use crate::activity::{Activity, BareActivity};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Executor};
use std::str::FromStr;

use crate::hasher;
use crate::user::{BareUser, User};

pub const DB_URI: &str = "data.db";

/// Error codes for the storage module
pub enum Error {
    ElementNotFound,
    InternalError,
    NotImplemented,
}

pub async fn init() -> Result<(), Error> {
    let connection: SqliteConnectOptions =
        match SqliteConnectOptions::from_str(&format!("sqlite://{}", DB_URI)) {
            Ok(connection) => connection,
            Err(_) => return Err(Error::InternalError),
        };

    let mut connection = match connection.create_if_missing(true).connect().await {
        Ok(connection) => connection,
        Err(_) => return Err(Error::InternalError),
    };

    // create users table
    match connection.execute(
        "CREATE TABLE IF NOT EXISTS 'users' (
        'id'	INTEGER UNIQUE,
        'name'	TEXT NOT NULL UNIQUE,
        'password_hash'	TEXT NOT NULL UNIQUE,
        PRIMARY KEY('id' AUTOINCREMENT))")
        .await
    {
        Ok(_) => {}
        Err(_) => return Err(Error::InternalError), // todo return e
    };

    // create activities table
    match connection.execute(
        "CREATE TABLE IF NOT EXISTS 'activities' (
    	'id'	INTEGER UNIQUE,
    	'start_time'	TEXT NOT NULL,
    	'end_time'	TEXT NOT NULL,
    	'amount'	INTEGER NOT NULL,
    	'activity_type'	TEXT NOT NULL,
    	'author_id'	INTEGER NOT NULL,
    	FOREIGN KEY('author_id') REFERENCES 'users'('id'),
    	PRIMARY KEY('id' AUTOINCREMENT))")
        .await
    {
        Ok(_) => {}
        Err(_) => return Err(Error::InternalError), // todo return e
    };

    Ok(())
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
        Err(_) => {
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
