use crate::schema::{students, teachers};
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use rocket_sync_db_pools::{database, diesel::SqliteConnection};

#[derive(Queryable, Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = teachers)]
#[serde(crate = "rocket::serde")]
pub struct Teacher {
    pub name: String,
    pub class: i32,
    pub course: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = students)]
#[serde(crate = "rocket::serde")]
pub struct Student {
    pub name: String,
    pub roll_number: i32,
    pub class: i32,
    pub english_score: i32,
    pub computer_score: i32,
    pub math_score: i32,
    pub sports_score: i32,
}

#[database("school")]
pub struct School(SqliteConnection);
