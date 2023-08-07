use super::entities::person::*;
use super::traits::PersonRepository;

pub struct Service {
    person_repository: Box<dyn PersonRepository>,
}

impl Service {
    pub fn new(person_repository: Box<dyn PersonRepository>) -> Service {
        Service { person_repository }
    }
    pub async fn create_person(&mut self, new_person: &NewPerson<'_>) -> Person {
        self.person_repository.create(new_person).await.unwrap()
    }

    pub async fn get_persons(&mut self) -> Vec<Person> {
        self.person_repository.get_all().await.unwrap()
    }

    pub async fn delete_person(&mut self, id: i64) {
        self.person_repository.delete(id).await.unwrap()
    }

    pub async fn get_person_by_id(&mut self, id: i64) -> Person {
        self.person_repository.get_by_id(id).await.unwrap()
    }
}
