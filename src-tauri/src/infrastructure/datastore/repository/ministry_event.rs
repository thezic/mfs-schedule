use async_mutex::Mutex;
use async_trait::async_trait;

use sqlx::FromRow;
use std::sync::Arc;

use crate::core::{entities::ministry_event::*, errors::*, traits};

pub struct MinistryEventRepository {
    conn: Arc<Mutex<super::TConn>>,
}

impl MinistryEventRepository {
    pub fn new(conn: Arc<Mutex<super::TConn>>) -> MinistryEventRepository {
        MinistryEventRepository { conn }
    }
}

#[derive(FromRow)]
struct MinistryEventRow {
    id: i64,
    assignee_name: String,
    assignee_id: Option<i64>,
    scheduled_time: String,
    place: String,
    extra_info: String,
}

impl From<chrono::ParseError> for DataStoreError {
    fn from(_error: chrono::ParseError) -> DataStoreError {
        DataStoreError::ParseError
    }
}

impl TryFrom<MinistryEventRow> for MinistryEvent {
    type Error = DataStoreError;
    fn try_from(value: MinistryEventRow) -> Result<Self, Self::Error> {
        Ok(MinistryEvent {
            id: value.id,
            assignee_name: value.assignee_name,
            assignee_id: value.assignee_id,
            extra_info: value.extra_info,
            place: value.place,
            scheduled_time: value.scheduled_time.parse()?,
        })
    }
}

#[async_trait]
impl traits::Repository<MinistryEvent, NewMinistryEvent> for MinistryEventRepository {
    async fn get_by_id(&self, id: i64) -> Result<MinistryEvent, DataStoreError> {
        let event_row = sqlx::query_as!(
            MinistryEventRow,
            "SELECT * FROM ministry_events WHERE id = $1",
            id
        )
        .fetch_one(&*self.conn.lock().await)
        .await?;

        let event = MinistryEvent::try_from(event_row)?;
        Ok(event)
    }

    async fn delete(&mut self, id: i64) -> Result<(), DataStoreError> {
        let conn = self.conn.lock().await;

        let result = sqlx::query!("DELETE FROM ministry_events WHERE id = ?", id)
            .execute(&*conn)
            .await?;

        if result.rows_affected() < 1 {
            return Err(DataStoreError::EntityNotFound);
        }

        Ok(())
    }

    async fn save(&mut self, entity: MinistryEvent) -> Result<MinistryEvent, DataStoreError> {
        {
            let result = sqlx::query!(
                r#"
                UPDATE ministry_events
                SET
                      assignee_name = ?
                    , assignee_id = ?
                    , scheduled_time = ?
                    , place = ?
                    , extra_info = ?
                WHERE
                    id = ?
                "#,
                entity.assignee_name,
                entity.assignee_id,
                entity.scheduled_time,
                entity.place,
                entity.extra_info,
                entity.id
            )
            .execute(&*self.conn.lock().await)
            .await?;

            if result.rows_affected() <= 1 {
                return Err(DataStoreError::EntityNotFound);
            }
        }

        Ok(self.get_by_id(entity.id).await?)
    }

    async fn get_all(&self) -> Result<Vec<MinistryEvent>, DataStoreError> {
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

    async fn create(
        &mut self,
        new_event: &NewMinistryEvent,
    ) -> Result<MinistryEvent, DataStoreError> {
        let id;
        {
            id = sqlx::query!(
                r#"
                INSERT INTO ministry_events (
                    assignee_name
                    , assignee_id
                    , scheduled_time
                    , place
                    , extra_info
                )
                VALUES (?, ?, ?, ?, ?)
                "#,
                new_event.assignee_name,
                new_event.assignee_id,
                new_event.scheduled_time,
                new_event.place,
                new_event.extra_info
            )
            .execute(&*self.conn.lock().await)
            .await?
            .last_insert_rowid();
        }

        Ok(self.get_by_id(id).await?)
    }
}

impl traits::MinistryEventRepository for MinistryEventRepository {}
