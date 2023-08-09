use chrono::prelude::*;

pub struct MinistryEvent {
    id: i64,
    assignee_name: String,
    assignee_id: Option<i64>,
    scheduled_time: DateTime<Utc>,
    place: String,
    extra_info: String,
}
