use crate::schema::*;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use rocket_sync_db_pools::{database, diesel::SqliteConnection};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = grades)]
#[serde(crate = "rocket::serde")]
pub struct Grade {
    pub grade_id: i32,
    pub student_id: i32,
    pub class_name: String,
    pub assignment_score: i32,
    pub test_score: i32,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = students)]
#[serde(crate = "rocket::serde")]
pub struct Student {
    pub student_id: i32,
    pub student_name: String,
    pub contact_info: String,
    pub email: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = subs)]
#[serde(crate = "rocket::serde")]
pub struct Sub {
    pub class_id: i32,
    pub class_name: String,
    pub teacher_id: i32,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = teachers)]
#[serde(crate = "rocket::serde")]
pub struct Teacher {
    pub teacher_id: i32,
    pub teacher_name: String,
    pub class_name: String,
    pub email: String,
}

#[database("school")]
pub struct School(SqliteConnection);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TBSError {
    err: String,
    code: i32,
}
