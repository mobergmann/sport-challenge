use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};

#[derive(Clone, Debug, Decode, Deserialize, Serialize, FromRow)]
pub struct Like {
    athlete_id: i64,
    activity_id: i64,
}
