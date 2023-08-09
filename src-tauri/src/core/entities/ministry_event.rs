use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MinistryEvent {
    #[specta(type = i32)]
    pub id: i64,
    pub assignee_name: String,
    #[specta(type = Option<i32>)]
    pub assignee_id: Option<i64>,
    pub scheduled_time: DateTime<Utc>,
    pub place: String,
    pub extra_info: String,
}

// impl From<
