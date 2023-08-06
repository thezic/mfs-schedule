use async_trait::async_trait;

use crate::core::entities::person::*;

#[async_trait]
pub trait PersonRepository {
    async fn get_all(&mut self) -> Result<Vec<Person>, anyhow::Error>;
    async fn create(&mut self, new_person: &NewPerson) -> Result<Person, anyhow::Error>;
}
