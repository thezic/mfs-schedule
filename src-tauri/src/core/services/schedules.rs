use crate::core::entities::ministry_event::*;
use crate::core::entities::person::*;
use crate::core::errors::DataStoreError;
use crate::core::traits::*;

pub struct Service {
    person_repository: Box<dyn PersonRepository>,
    event_repository: Box<dyn MinistryEventRepository>,
}

impl Service {
    pub fn new(
        person_repository: Box<dyn PersonRepository>,
        event_repository: Box<dyn MinistryEventRepository>,
    ) -> Service {
        Service {
            person_repository,
            event_repository,
        }
    }
    pub async fn create_person(
        &mut self,
        new_person: &NewPerson<'_>,
    ) -> Result<Person, DataStoreError> {
        self.person_repository.create(new_person).await
    }

    pub async fn get_persons(&mut self) -> Result<Vec<Person>, DataStoreError> {
        self.person_repository.get_all().await
    }

    pub async fn delete_person(&mut self, id: i64) -> Result<(), DataStoreError> {
        self.person_repository.delete(id).await
    }

    pub async fn get_person_by_id(&mut self, id: i64) -> Result<Person, DataStoreError> {
        self.person_repository.get_by_id(id).await
    }

    pub async fn update_person(&mut self, person: Person) -> Result<Person, DataStoreError> {
        self.person_repository.save(person).await
    }

    pub async fn get_planned_events(&mut self) -> Result<Vec<MinistryEvent>, DataStoreError> {
        self.event_repository.get_all().await
    }

    pub async fn create_event(
        &mut self,
        new_event: &NewMinistryEvent,
    ) -> Result<MinistryEvent, DataStoreError> {
        self.event_repository.create(new_event).await
    }

    pub async fn update_event(
        &mut self,
        event: MinistryEvent,
    ) -> Result<MinistryEvent, DataStoreError> {
        self.event_repository.save(event).await
    }

    pub async fn delete_event(&mut self, id: i64) -> Result<(), DataStoreError> {
        self.event_repository.delete(id).await
    }
}
