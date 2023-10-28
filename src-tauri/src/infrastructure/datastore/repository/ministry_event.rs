use async_mutex::Mutex;
use async_trait::async_trait;

use chrono::NaiveTime;
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

#[derive(Debug, FromRow)]
struct MinistryEventRow {
    id: i64,
    assignee_name: String,
    assignee_id: Option<i64>,
    date: String,
    time: Option<String>,
    place: String,
    extra_info: String,
}

impl From<chrono::ParseError> for DataStoreError {
    fn from(error: chrono::ParseError) -> DataStoreError {
        DataStoreError::ParseError {
            error: error.to_string(),
            value: "(Unknown)".to_string(),
        }
    }
}

fn parse_time(v: &str) -> Result<NaiveTime, DataStoreError> {
    NaiveTime::parse_from_str(v, "%H:%M").map_err(|error| DataStoreError::ParseError {
        value: v.to_string(),
        error: error.to_string(),
    })
}

impl TryFrom<MinistryEventRow> for MinistryEvent {
    type Error = DataStoreError;
    fn try_from(value: MinistryEventRow) -> Result<Self, Self::Error> {
        let time = match value.time {
            Some(time_string) => Some(parse_time(&time_string)?),
            None => None,
        };

        Ok(MinistryEvent {
            id: value.id,
            assignee_name: value.assignee_name,
            assignee_id: value.assignee_id,
            extra_info: value.extra_info,
            place: value.place,
            date: value.date.parse()?,
            time,
        })
    }
}

struct MinistryEventRowVec(Vec<MinistryEventRow>);

impl TryFrom<MinistryEventRowVec> for Vec<MinistryEvent> {
    type Error = DataStoreError;
    fn try_from(value: MinistryEventRowVec) -> Result<Self, Self::Error> {
        let result: Result<Vec<_>, _> = value
            .0
            .into_iter()
            .map(|row| -> Result<MinistryEvent, Self::Error> { MinistryEvent::try_from(row) })
            .collect();

        result
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
            let time = entity.time.map(|t| t.format("%H:%M").to_string());

            let result = sqlx::query!(
                r#"
                UPDATE ministry_events
                SET
                      assignee_name = ?
                    , assignee_id = ?
                    , date = ?
                    , time = ?
                    , place = ?
                    , extra_info = ?
                WHERE
                    id = ?
                "#,
                entity.assignee_name,
                entity.assignee_id,
                entity.date,
                time,
                entity.place,
                entity.extra_info,
                entity.id
            )
            .execute(&*self.conn.lock().await)
            .await?;

            if result.rows_affected() == 0 {
                return Err(DataStoreError::EntityNotFound);
            }
        }

        Ok(self.get_by_id(entity.id).await?)
    }

    async fn get_all(&self) -> Result<Vec<MinistryEvent>, DataStoreError> {
        let result = sqlx::query_as!(
            MinistryEventRow,
            r#"
            SELECT * FROM ministry_events
            "#
        )
        .fetch_all(&*self.conn.lock().await)
        .await?;

        Ok(MinistryEventRowVec(result).try_into()?)
    }

    async fn create(
        &mut self,
        new_event: &NewMinistryEvent,
    ) -> Result<MinistryEvent, DataStoreError> {
        let time = new_event.time.map(|t| t.format("%H:%M").to_string());
        let id;

        {
            // Scope releases mutex after use
            id = sqlx::query!(
                r#"
                INSERT INTO ministry_events (
                    assignee_name
                    , assignee_id
                    , date
                    , time
                    , place
                    , extra_info
                )
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                new_event.assignee_name,
                new_event.assignee_id,
                new_event.date,
                time,
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

#[async_trait]
impl traits::MinistryEventRepository for MinistryEventRepository {
    async fn get_range(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<MinistryEvent>, DataStoreError> {
        let result = sqlx::query_as!(
            MinistryEventRow,
            r#"
            SELECT * FROM ministry_events
            WHERE 
                date BETWEEN ? and ?
            ORDER BY date
            "#,
            from,
            to
        )
        .fetch_all(&*self.conn.lock().await)
        .await?;

        Ok(MinistryEventRowVec(result).try_into()?)
    }
}
