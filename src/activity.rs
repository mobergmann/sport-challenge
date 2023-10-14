use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
pub enum ActivityType {
    PushUps,
}

#[derive(Clone, Debug, Decode, Deserialize, Serialize, FromRow)]
pub struct Activity {
    pub id: i64,
    pub author_id: i64,
    pub amount: i64,
    pub activity_type: ActivityType,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BareActivity {
    pub amount: i64,
    pub activity_type: ActivityType,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
}
