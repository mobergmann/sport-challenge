use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, SqlitePool};
use std::str::FromStr;

/// Path to the SQLite database
const DB_URI: &str = "sqlite://data.db";

/// Error enum also wrapping sqlx errors
pub enum Error {
    SQLX(sqlx::Error),
    NotImplemented,
    Conflict(String),
}

/// For when an `sqlx::Error` is thrown we can convert it implicitly into an `database::Error`
impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::SQLX(error)
    }
}

/// Initialize the database.
/// If the SQLite-File does not exist create it and create the tables.
pub async fn init() -> Result<SqlitePool, sqlx::Error> {
    let pool_options = SqliteConnectOptions::from_str(DB_URI)?
        //.filename(DB_URI)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(pool_options).await?;

    let mut connection = pool.acquire().await?;

    // create users table
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS 'users' (
        'id'	INTEGER UNIQUE,
        'username'	TEXT NOT NULL UNIQUE,
        'password_hash'	TEXT NOT NULL UNIQUE,
        PRIMARY KEY('id' AUTOINCREMENT))",
        )
        .await?;

    // create activities table
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS 'activities' (
    	'id'	INTEGER UNIQUE,
    	'start_time'	TEXT NOT NULL,
    	'end_time'	TEXT NOT NULL,
    	'amount'	INTEGER NOT NULL,
    	'activity_type'	TEXT NOT NULL,
    	'author_id'	INTEGER NOT NULL,
    	FOREIGN KEY('author_id') REFERENCES 'users'('id'),
    	PRIMARY KEY('id' AUTOINCREMENT))",
        )
        .await?;

    Ok(pool)
}

pub mod account {
    use crate::account::{Account, BareAccount, EditAccount};
    use crate::database::Error;
    use crate::hasher;
    use sqlx::SqlitePool;

    /// Returns an account by id
    pub async fn get_id(pool: SqlitePool, id: i64) -> Result<Account, Error> {
        let mut connection = pool.acquire().await?;

        let user: Account = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&mut connection)
            .await?;

        Ok(user)
    }

    /// Returns an account by name
    pub async fn get(pool: SqlitePool, username: &String) -> Result<Account, Error> {
        let mut connection = pool.acquire().await?;

        let user: Account = sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&mut connection)
            .await?;

        Ok(user)
    }

    /// Inserts an Account and returns the inserted account
    pub async fn insert(pool: SqlitePool, account: &BareAccount) -> Result<Account, Error> {
        let mut connection = pool.acquire().await?;

        let password_hash = hasher::hash(&account.password);

        let user: Account =
            sqlx::query_as("INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING *")
                .bind(&account.username)
                .bind(password_hash)
                .fetch_one(&mut connection)
                .await?;

        Ok(user)
    }

    /// Updates an account and returns the updated account
    pub async fn update(
        pool: SqlitePool,
        id: i64,
        account: &EditAccount,
    ) -> Result<Account, sqlx::Error> {
        let mut connection = pool.acquire().await?;

        let result: Account = sqlx::query_as("UPDATE users SET username = $1 WHERE id = $2 RETURNING *")
            .bind(&account.username)
            .bind(id)
            .fetch_one(&mut connection)
            .await?;

        Ok(result)
    }

    /// Updates the password of the current account and returns the updated account
    pub async fn update_password(
        pool: SqlitePool,
        id: i64,
        new_password: String,
    ) -> Result<Account, sqlx::Error> {
        let mut connection = pool.acquire().await?;

        let password_hash = hasher::hash(&new_password);

        let user: Account =
            sqlx::query_as("UPDATE users SET password_hash = $1 WHERE id = $2 RETURNING *")
                .bind(password_hash)
                .bind(id)
                .fetch_one(&mut connection)
                .await?;

        Ok(user)
    }

    /// Deletes an Account and returns the deleted account
    pub async fn delete(pool: SqlitePool, id: i64) -> Result<Account, Error> {
        let mut connection = pool.acquire().await?;

        // todo can delete return
        let user: Account =
            sqlx::query_as("DELETE FROM users WHERE id = $1 RETURNING *")
                .bind(id)
                .fetch_one(&mut connection)
                .await?;

        Ok(user)
    }
}

pub mod user {
    use crate::account::{Account, User};
    use crate::database::Error;
    use sqlx::SqlitePool;

    /// Returns a user by username
    pub async fn get(pool: SqlitePool, username: &String) -> Result<User, Error> {
        let mut connection = pool.acquire().await?;

        let user: Account = sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username.as_str())
            .fetch_one(&mut connection)
            .await?;

        Ok(From::from(user))
    }

    /// Returns a user by id
    pub async fn get_id(pool: SqlitePool, id: i64) -> Result<User, Error> {
        let mut connection = pool.acquire().await?;

        let user: Account = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&mut connection)
            .await?;

        Ok(From::from(user))
    }

    /// Checks weather a account/user (given by its username) exists
    pub async fn exists(pool: SqlitePool, username: &String) -> bool {
        let user = get(pool, username).await;
        match user {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Checks weather a account/user (given by its is) exists
    pub async fn exists_id(pool: SqlitePool, id: i64) -> bool {
        let user = get_id(pool, id).await;
        match user {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

pub mod activity {
    use crate::activity::{Activity, BareActivity};
    use crate::database::Error;
    use chrono::{DateTime, Utc};
    use sqlx::SqlitePool;

    /// Returns an activity
    pub async fn get(pool: SqlitePool, id: i64) -> Result<Activity, Error> {
        let mut connection = pool.acquire().await?;

        let activity: Activity = sqlx::query_as("SELECT * FROM activities WHERE id = $1")
            .bind(id)
            .fetch_one(&mut connection)
            .await?;

        Ok(activity)
    }

    /// Returns a list of Activities which took place in the time interval from :from to :to
    pub async fn get_interval(
        pool: SqlitePool,
        from: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Vec<Activity>, Error> {
        let mut connection = pool.acquire().await?;

        let activities: Vec<Activity> =
            sqlx::query_as("SELECT * FROM activities WHERE start_time >= $1 AND end_time <= $2")
                .bind(from)
                .bind(to)
                .fetch_all(&mut connection)
                .await?;

        Ok(activities)
    }

    /// Inserts an activity into the database and returns the newly inserted activity
    pub async fn insert(
        pool: SqlitePool,
        author_id: i64,
        activity: &BareActivity,
    ) -> Result<Activity, Error> {
        let mut connection = pool.acquire().await?;

        let activity: Activity = sqlx::query_as("INSERT INTO activities (author_id, amount, activity_type, start_time, end_time) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(author_id)
            .bind(activity.amount)
            .bind(&activity.activity_type)
            .bind(activity.start_time)
            .bind(activity.end_time)
            .fetch_one(&mut connection)
            .await?;

        Ok(activity)
    }

    /// Updates an activity and returns the updated activity
    pub async fn update(
        pool: SqlitePool,
        id: i64,
        activity: &BareActivity,
    ) -> Result<Activity, Error> {
        let mut connection = pool.acquire().await?;

        let result: Activity = sqlx::query_as("UPDATE activities SET amount = $1, activity_type = $2, start_time = $3, end_time = $4 WHERE id = $5 RETURNING *")
            .bind(activity.amount)
            .bind(&activity.activity_type)
            .bind(activity.start_time)
            .bind(activity.end_time)
            .bind(id)
            .fetch_one(&mut connection)
            .await?;

        Ok(result)
    }

    /// Deletes an activity and returns the deleted activity
    pub async fn delete(pool: SqlitePool, id: i64) -> Result<Activity, Error> {
        let mut connection = pool.acquire().await?;

        let activity: Activity = sqlx::query_as("DELETE FROM activities WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&mut connection)
            .await?;

        Ok(activity)
    }
}
