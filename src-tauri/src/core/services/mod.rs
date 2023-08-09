use super::entities::person::*;
use super::errors::DataStoreError;
use super::traits::PersonRepository;

pub struct Service {
    person_repository: Box<dyn PersonRepository>,
}

impl Service {
    pub fn new(person_repository: Box<dyn PersonRepository>) -> Service {
        Service { person_repository }
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
}
