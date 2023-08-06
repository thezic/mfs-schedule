#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Person {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewPerson<'a> {
    pub name: &'a str,
}
