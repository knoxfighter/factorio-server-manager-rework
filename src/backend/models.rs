use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}
