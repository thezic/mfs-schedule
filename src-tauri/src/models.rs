use crate::schema::persons;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = persons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = persons)]
pub struct NewPerson<'a> {
    pub name: &'a str,
}
