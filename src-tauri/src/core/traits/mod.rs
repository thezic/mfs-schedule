use async_trait::async_trait;

use super::errors::DataStoreError;
use crate::core::entities::person::*;

#[async_trait]
pub trait PersonRepository: std::marker::Send {
    async fn get_all(&mut self) -> Result<Vec<Person>, DataStoreError>;
    async fn create(&mut self, new_person: &NewPerson) -> Result<Person, DataStoreError>;
    async fn delete(&mut self, id: i64) -> Result<(), DataStoreError>;
    async fn get_by_id(&mut self, id: i64) -> Result<Person, DataStoreError>;
    async fn save(&mut self, person: Person) -> Result<Person, DataStoreError>;
}
