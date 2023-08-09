use async_mutex::Mutex;
use async_trait::async_trait;
use chrono::prelude::*;
use sqlx::Row;
use std::{str::FromStr, sync::Arc};

use crate::core::{entities::ministry_event::*, errors::*, traits};

pub struct MinistryEventRepository {
    conn: Arc<Mutex<super::TConn>>,
}

impl MinistryEventRepository {
    pub fn new(conn: Arc<Mutex<super::TConn>>) -> MinistryEventRepository {
        MinistryEventRepository { conn }
    }
}

#[async_trait]
impl traits::MinistryEventRepository for MinistryEventRepository {
    async fn get_all(&mut self) -> Result<Vec<MinistryEvent>, DataStoreError> {
        let result = sqlx::query!(
            r#"
            SELECT 
                id
                , assignee_name
                , assignee_id
                , scheduled_time
                , place
                , extra_info 
            FROM ministry_events
            "#
        )
        .map(|row| MinistryEvent {
            id: row.id,
            assignee_id: row.assignee_id,
            extra_info: row.extra_info,
            place: row.place,
            assignee_name: row.assignee_name,
            scheduled_time: row.scheduled_time.parse().unwrap(),
        })
        .fetch_all(&*self.conn.lock().await)
        .await?;

        Ok(result)
    }
}
