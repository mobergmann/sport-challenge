use std::str::FromStr;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Executor, SqlitePool};

/// Path to the SQLite database
pub const DB_URI: &str = "sqlite://data.db";

static pool: SqlitePool = {
    let connect_options = SqliteConnectOptions::new()
        .filename(DB_URI)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(connect_options).expect("");
    pool
};

/// Initialize the database.
/// If the SQLite-File does not exist create it and create the tables.
pub async fn init() -> Result<(), sqlx::Error> {

    let mut connection = pool.acquire().await?;

    // create users table
    connection.execute(
        "CREATE TABLE IF NOT EXISTS 'users' (
        'id'	INTEGER UNIQUE,
        'name'	TEXT NOT NULL UNIQUE,
        'password_hash'	TEXT NOT NULL UNIQUE,
        PRIMARY KEY('id' AUTOINCREMENT))")
        .await?;

    // create activities table
    connection.execute(
        "CREATE TABLE IF NOT EXISTS 'activities' (
    	'id'	INTEGER UNIQUE,
    	'start_time'	TEXT NOT NULL,
    	'end_time'	TEXT NOT NULL,
    	'amount'	INTEGER NOT NULL,
    	'activity_type'	TEXT NOT NULL,
    	'author_id'	INTEGER NOT NULL,
    	FOREIGN KEY('author_id') REFERENCES 'users'('id'),
    	PRIMARY KEY('id' AUTOINCREMENT))")
        .await?;

    Ok(pool)
}

mod account {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;
    use crate::account::{Account, BareAccount, EditAccount};
    use crate::database::{DB_URI, Error};
    use crate::hasher;

    /// Returns an account by id
    pub async fn get(id: i64) -> Result<Account, sqlx::Error> {
        let mut connection = pool.acquire().await?;

        let user: Account = sqlx::query_as("select * from users where id = $1")
            .bind(id)
            .fetch_one(&mut connection)
            .await?;

        Ok(user)
    }

    /// Returns an account by name
    pub async fn get_name(pool: SqlitePool, username: &String) -> Result<Account, sqlx::Error> {
        let mut connection = pool.acquire().await?;

        let user: Account = sqlx::query_as("select * from users where name = $1")
            .bind(username)
            .fetch_one(&pool)
            .await?;

        Ok(user)
    }

    /// Inserts an Account and returns the inserted account
    pub async fn insert(account: BareAccount) -> Result<Account, Error> {
        let password_hash = hasher::hash(&account.password);

        let mut connection = pool.acquire().await?;

        let user: Account = sqlx::query_as("insert into users (name, password_hash) values ($1, $2) returning *")
            .bind(&account.name).bind(password_hash)
            .fetch_one(&pool)
            .await
            .unwrap();

        Ok(user)
    }

    /// Updates an account and returns the updated account
    pub async fn update(id: i64, account: EditAccount) -> Result<Account, Error> {
        Err(Error::NotImplemented)
    }

    /// Deletes an Account and returns the deleted account
    pub async fn delete(id: i64) -> Result<Account, Error> {
        Err(Error::NotImplemented)
    }
}

mod user {
    use crate::account::User;
    use crate::logic::users::get_user_id;

    /// Returns a user by username
    pub async fn get(username: &String) -> Result<User, Error> {
        let mut connection = pool.acquire().await?;

        let user: Account = match sqlx::query_as("select * from users where name = $1")
            .bind(username.as_str())
            .fetch_one(&pool)
            .await
        {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => return Err(Error::ElementNotFound),
            Err(_) => return Err(Error::InternalError), // todo return e
        };

        Ok(user)
    }

    /// Returns a user by id
    pub async fn get_id(id: &i64)  -> Result<User, Error> {
        let mut connection = pool.acquire().await?;

        let user: Account = match sqlx::query_as("select * from users where id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
        {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => return Err(Error::ElementNotFound),
            Err(_) => return Err(Error::InternalError), // todo return e
        };

        Ok(user)
    }

    pub async fn exists(name: &String) -> bool {
        let user = get(name).await;
        match user {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn exists_id(id: &i64) -> bool {
        let user = get_id(id).await;
        match user {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

mod activity {
    use chrono::{DateTime, Utc};
    use sqlx::sqlite::SqlitePoolOptions;
    use crate::activity::{Activity, BareActivity};
    use crate::database::{DB_URI, Error};

    /// Returns an activity
    pub async fn get(id: i64) -> Result<Activity, Error> {
        let mut connection = pool.acquire().await?;

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

    /// Returns a list of Activities which took place in the time interval from :from to :to
    pub async fn get_interval(from: DateTime<Utc>, to: DateTime<Utc>) ->  Result<Vec<Activity>, Error> {
        let mut connection = pool.acquire().await?;

        match sqlx::query_as("SELECT * FROM activities WHERE start_time >= $1 and end_time <= $2")
            .bind(from)
            .bind(to)
            .fetch_all(&pool)
            .await
        {
            Ok(activities) => Ok(activities),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Inserts an activity into the database and returns the newly inserted activity
    pub async fn insert(activity: BareActivity) -> Result<Activity, Error> {
        let mut connection = pool.acquire().await?;

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

    /// Updates an activity and returns the updated activity
    pub async fn update(id: i64, activity: BareActivity) -> Result<Activity, Error> {
        Err(Error::NotImplemented)
    }

    /// Deletes an activity and returns the deleted activity
    pub async fn delete(id: i64) -> Result<Activity, Error> {
        let mut connection = pool.acquire().await?;

        let activity: Activity = sqlx::query_as("delete from activities where id = $1 returning *")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();

        Ok(activity)
    }
}
