use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
pub enum ActivityType {
    Running,
    Bicycling,
    PushUps,
}

#[derive(Clone, Debug, Decode, Deserialize, Serialize, FromRow)]
pub struct Activity {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub description: String,
    pub amount: i64,
    pub activity_type: ActivityType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BareActivity {
    pub title: String,
    pub description: String,
    pub amount: i64,
    pub activity_type: ActivityType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringBareActivity {
    pub title: String,
    pub description: String,
    pub amount: i64,
    pub activity_type: ActivityType,
    pub start_time: String,
    pub end_time: String,
}
