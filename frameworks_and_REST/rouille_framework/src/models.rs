use super::schema::users;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
