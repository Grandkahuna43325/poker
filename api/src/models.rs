use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::soul)]
pub struct Soul {
    pub id: i32,
    pub owner: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Identifiable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::player)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub score: i32,
}

#[derive(Debug, Queryable, Identifiable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::admin)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password: String,
}
