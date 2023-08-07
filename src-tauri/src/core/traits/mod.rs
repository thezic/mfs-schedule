use async_trait::async_trait;

use crate::core::entities::person::*;

#[async_trait]
pub trait PersonRepository: std::marker::Send {
    async fn get_all(&mut self) -> Result<Vec<Person>, anyhow::Error>;
    async fn create(&mut self, new_person: &NewPerson) -> Result<Person, anyhow::Error>;
    async fn delete(&mut self, id: i64) -> Result<(), anyhow::Error>;
    async fn get_by_id(&mut self, id: i64) -> Result<Person, anyhow::Error>;
}
