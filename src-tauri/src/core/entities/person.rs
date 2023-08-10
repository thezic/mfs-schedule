use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Person {
    #[specta(type = i32)]
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NewPerson<'a> {
    pub name: &'a str,
}
