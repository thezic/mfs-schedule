use async_trait::async_trait;

use crate::core::{
    entities::person::{NewPerson, Person},
    traits,
};

type TConn = sqlx::pool::PoolConnection<sqlx::Sqlite>;

pub struct PersonRepository {
    // db: &'a sqlx::SqlitePool,
    conn: TConn,
}

impl PersonRepository {
    pub fn new(conn: TConn) -> PersonRepository {
        PersonRepository { conn }
    }
}

#[async_trait]
impl traits::PersonRepository for PersonRepository {
    async fn get_all(&mut self) -> Result<Vec<Person>, anyhow::Error> {
        // let mut connection = self.db.acquire().await?;

        let result = sqlx::query_as!(Person, "SELECT * FROM persons")
            .fetch_all(&mut *self.conn)
            .await?;

        Ok(result)
    }

    async fn create(&mut self, new_person: &NewPerson) -> Result<Person, anyhow::Error> {
        // let mut conn = self.db.acquire().await?;

        let id = sqlx::query!("INSERT INTO persons (name) VALUES ($1)", new_person.name)
            .execute(&mut *self.conn)
            .await?
            .last_insert_rowid();

        let person = sqlx::query_as!(Person, "SELECT * FROM persons WHERE id = $1", id)
            .fetch_one(&mut *self.conn)
            .await?;
        // let person = self.get_by_id(id).await?;
        Ok(person)
    }

    async fn delete(&mut self, id: i64) -> Result<(), anyhow::Error> {
        sqlx::query!("DELETE FROM persons WHERE id = $1", id)
            .execute(&mut *self.conn)
            .await?;
        Ok(())
    }

    async fn get_by_id(&mut self, id: i64) -> Result<Person, anyhow::Error> {
        // let mut conn = self.db.acquire().await?;
        let person = sqlx::query_as!(Person, "SELECT * FROM persons WHERE id = $1", id)
            .fetch_one(&mut *self.conn)
            .await?;
        Ok(person)
    }
}
