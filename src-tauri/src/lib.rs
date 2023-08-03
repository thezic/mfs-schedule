use self::models::{NewPerson, Person};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn connect() -> SqliteConnection {
    let database_url = "database.sqlite";
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database: {database_url}"))
}

pub fn create_person(conn: &mut SqliteConnection, name: &str) {
    use self::schema::persons;
    let person = NewPerson { name };

    diesel::insert_into(persons::table)
        .values(&person)
        // .returning(Person::as_returning())
        .execute(conn)
        .expect("Error saving new person");
}

pub fn get_persons(conn: &mut SqliteConnection) -> Vec<String> {
    use self::schema::persons::dsl::*;
    let results = persons
        .select(Person::as_select())
        .load(conn)
        .expect("Error loading persons");

    results.into_iter().map(|p| p.name).collect()
    // vec![String::from("Hello")]
}

pub mod models;
pub mod schema;
