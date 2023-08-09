use async_trait::async_trait;

use crate::core::{
    entities::person::{NewPerson, Person},
    errors::*,
    traits,
};

type TConn = sqlx::pool::PoolConnection<sqlx::Sqlite>;

pub struct PersonRepository {
    conn: TConn,
}

impl PersonRepository {
    pub fn new(conn: TConn) -> PersonRepository {
        PersonRepository { conn }
    }
}

#[async_trait]
impl traits::PersonRepository for PersonRepository {
    async fn get_all(&mut self) -> Result<Vec<Person>, DataStoreError> {
        let result = sqlx::query_as!(Person, "SELECT * FROM persons")
            .fetch_all(&mut *self.conn)
            .await?;

        Ok(result)
    }

    async fn create(&mut self, new_person: &NewPerson) -> Result<Person, DataStoreError> {
        let id = sqlx::query!("INSERT INTO persons (name) VALUES ($1)", new_person.name)
            .execute(&mut *self.conn)
            .await?
            .last_insert_rowid();

        Ok(self.get_by_id(id).await?)
    }

    async fn delete(&mut self, id: i64) -> Result<(), DataStoreError> {
        let result = sqlx::query!("DELETE FROM persons WHERE id = $1", id)
            .execute(&mut *self.conn)
            .await?;

        println!("Delete result: {}", result.rows_affected());
        if result.rows_affected() == 0 {
            return Err(DataStoreError::EntityNotFound);
        }

        Ok(())
    }

    async fn get_by_id(&mut self, id: i64) -> Result<Person, DataStoreError> {
        let person = sqlx::query_as!(Person, "SELECT * FROM persons WHERE id = $1", id)
            .fetch_one(&mut *self.conn)
            .await?;
        Ok(person)
    }

    async fn save(&mut self, person: Person) -> Result<Person, DataStoreError> {
        sqlx::query!(
            "UPDATE persons SET name = $1 WHERE id = $2",
            person.name,
            person.id,
        )
        .execute(&mut *self.conn)
        .await?;

        // Fetch and return saved person
        Ok(self.get_by_id(person.id).await?)
    }
}
