use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MinistryEvent {
    #[specta(type = i32)]
    id: i64,
    assignee_name: String,
    #[specta(type = Option<i32>)]
    assignee_id: Option<i64>,
    scheduled_time: DateTime<Utc>,
    place: String,
    extra_info: String,
}
