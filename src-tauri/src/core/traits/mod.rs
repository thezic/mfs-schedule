use async_trait::async_trait;

use super::errors::DataStoreError;
use crate::core::entities::ministry_event::*;
use crate::core::entities::person::*;

#[async_trait]
pub trait Repository<TEntity, TNewEntity>: std::marker::Send + std::marker::Sync {
    async fn get_all(&self) -> Result<Vec<TEntity>, DataStoreError>;
    async fn get_by_id(&self, id: i64) -> Result<TEntity, DataStoreError>;
    async fn create(&mut self, new_event: &TNewEntity) -> Result<TEntity, DataStoreError>;
    async fn save(&mut self, entity: TEntity) -> Result<TEntity, DataStoreError>;
    async fn delete(&mut self, id: i64) -> Result<(), DataStoreError>;
}

#[async_trait]
pub trait PersonRepository: std::marker::Send {
    async fn get_all(&mut self) -> Result<Vec<Person>, DataStoreError>;
    async fn create(&mut self, new_person: &NewPerson) -> Result<Person, DataStoreError>;
    async fn delete(&mut self, id: i64) -> Result<(), DataStoreError>;
    async fn get_by_id(&mut self, id: i64) -> Result<Person, DataStoreError>;
    async fn save(&mut self, person: Person) -> Result<Person, DataStoreError>;
}

#[async_trait]
pub trait MinistryEventRepository: Repository<MinistryEvent, NewMinistryEvent> {
    async fn get_range(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<MinistryEvent>, DataStoreError>;

    // async fn get_all(&mut self) -> Result<Vec<MinistryEvent>, DataStoreError>;
    // async fn create(
    //     &mut self,
    //     new_event: &NewMinistryEvent,
    // ) -> Result<MinistryEvent, DataStoreError>;
}
