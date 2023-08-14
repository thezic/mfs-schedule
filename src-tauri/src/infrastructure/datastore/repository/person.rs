use async_mutex::Mutex;
use async_trait::async_trait;
use std::sync::Arc;

use crate::core::{
    entities::person::{NewPerson, Person},
    errors::*,
    traits,
};

pub struct PersonRepository {
    // conn: Arc<Mutex<sqlx::SqliteConnection>>,
    conn: Arc<Mutex<sqlx::pool::Pool<sqlx::Sqlite>>>,
}

impl PersonRepository {
    pub fn new(conn: Arc<Mutex<sqlx::pool::Pool<sqlx::Sqlite>>>) -> PersonRepository {
        PersonRepository { conn }
    }

    // pub fn get_connection(&self) -> sqlx::pool::Pool<sqlx::Sqlite> {
    //     *self.conn.lock().unwrap()
    //     // *self.conn.lock().unwrap()
    // }
}

#[async_trait]
impl traits::PersonRepository for PersonRepository {
    async fn get_all(&mut self) -> Result<Vec<Person>, DataStoreError> {
        let guard = self.conn.lock_arc().await;
        let result = sqlx::query_as!(Person, "SELECT * FROM persons")
            .fetch_all(&*guard)
            .await?;

        Ok(result)
    }

    async fn create(&mut self, new_person: &NewPerson) -> Result<Person, DataStoreError> {
        let id = sqlx::query!(
            "INSERT INTO persons (name, comment) VALUES (?, ?)",
            new_person.name,
            new_person.comment
        )
        .execute(&*self.conn.lock_arc().await)
        .await?
        .last_insert_rowid();

        Ok(self.get_by_id(id).await?)
    }

    async fn delete(&mut self, id: i64) -> Result<(), DataStoreError> {
        let result = sqlx::query!("DELETE FROM persons WHERE id = $1", id)
            .execute(&*self.conn.lock_arc().await)
            .await?;

        println!("Delete result: {}", result.rows_affected());
        if result.rows_affected() == 0 {
            return Err(DataStoreError::EntityNotFound);
        }

        Ok(())
    }

    async fn get_by_id(&mut self, id: i64) -> Result<Person, DataStoreError> {
        let person = sqlx::query_as!(Person, "SELECT * FROM persons WHERE id = $1", id)
            .fetch_one(&*self.conn.lock_arc().await)
            .await?;
        Ok(person)
    }

    async fn save(&mut self, person: Person) -> Result<Person, DataStoreError> {
        sqlx::query!(
            "UPDATE persons SET name = ?, comment=? WHERE id = ?",
            person.name,
            person.comment,
            person.id,
        )
        .execute(&*self.conn.lock_arc().await)
        .await?;

        // Fetch and return saved person
        Ok(self.get_by_id(person.id).await?)
    }
}
