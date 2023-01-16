mod auth;
mod models;
mod schema;
mod swagger;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use auth::*;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::*;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::swagger_ui::make_swagger_ui;
use rocket_okapi::{openapi, openapi_get_routes};
use schema::*;
use std::env;
use swagger::swag_config;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

#[openapi(tag = "Home")]
#[get("/")]
fn home_page() -> &'static str {
    "Welcome To The Boring School"
}

// ACCESSIBLE TO: Principal & Teacher
#[openapi(tag = "AddOp")]
#[post("/add_student", format = "json", data = "<new_student>")]
async fn add_student(
    auth: Claims,
    new_student: Json<Student>,
) -> Result<status::Created<String>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            let c = establish_connection();

            let res = diesel::insert_into(students::table)
                .values(new_student.into_inner())
                .execute(&c);

            if res == Ok(1) {
                Ok(status::Created::new(
                    "Student successfully added to the School",
                ))
            } else {
                Err(status::Unauthorized(Some(String::from(
                    "You are not allowed to do that action without proper auth token",
                ))))
            }
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are not authorized to do that",
        )))),
    }
}

// ACCESSIBLE TO: Principal
#[openapi(tag = "AddOp")]
#[post("/add_teacher", format = "json", data = "<new_teacher>")]
async fn add_teacher(
    auth: Claims,
    new_teacher: Json<Teacher>,
) -> Result<status::Created<String>, status::Unauthorized<String>> {
    match auth.id {
        3 => {
            let c = establish_connection();

            let res = diesel::insert_into(teachers::table)
                .values(new_teacher.into_inner())
                .execute(&c);

            if res == Ok(1) {
                Ok(status::Created::new("Teacher added Successfully"))
            } else {
                Err(status::Unauthorized(Some(String::from(
                    "You are not allowed to do that",
                ))))
            }
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are not authorized to do that",
        )))),
    }
}

// ACCESSIBLE TO: PRINCIPAL & TEACHER
#[openapi(tag = "AddOp")]
#[post("/add_grade", format = "json", data = "<grade_record>")]
async fn add_grade(
    auth: Claims,
    grade_record: Json<Grade>,
) -> Result<status::Created<String>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            let c = establish_connection();

            let res = diesel::insert_into(grades::table)
                .values(grade_record.into_inner())
                .execute(&c);
            if res == Ok(1) {
                Ok(status::Created::new("Grade added successfully"))
            } else {
                Err(status::Unauthorized(Some(String::from(
                    "You are not allowed to do that",
                ))))
            }
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are not allowed to do that",
        )))),
    }
}

// ACCESSIBLE TO: ALL
#[openapi(tag = "GetOp")]
#[get("/all_teachers")]
async fn get_all_teachers() -> Json<Vec<Teacher>> {
    let c = establish_connection();
    Json(teachers::table.load::<Teacher>(&c).unwrap())
}

// ACCESSIBLE TO: Principal & Teacher
#[openapi(tag = "GetOp")]
#[get("/all_students")]
async fn get_all_student(
    auth: Claims,
) -> Result<Json<Vec<(String, i32)>>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            let c = establish_connection();
            Ok(Json(
                students::table
                    .select((students::student_name, students::student_id))
                    .load::<(String, i32)>(&c)
                    .unwrap(),
            ))
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are not allowed to do that",
        )))),
    }
}

// ACCESSIBLE TO: ALL
#[openapi(tag = "GetOp")]
#[get("/student/<student_id>")]
async fn get_student(student_id: i32) -> Json<Option<Student>> {
    let c = establish_connection();
    Json(
        students::table
            .filter(students::student_id.eq(student_id))
            .first::<Student>(&c)
            .ok(),
    )
}

#[openapi(tag = "GetOp")]
#[get("/grades/<student_id>")]
async fn get_grades(student_id: i32) -> Json<Vec<Grade>> {
    let c = establish_connection();
    Json(
        grades::table
            .filter(grades::student_id.eq(student_id))
            .load(&c)
            .unwrap(),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(School::fairing())
        .mount("/swagger", make_swagger_ui(&swag_config()))
        .mount(
            "/",
            openapi_get_routes![
                home_page,
                add_student,
                add_teacher,
                add_grade,
                get_all_student,
                get_all_teachers,
                get_student,
                get_grades,
            ],
        )
}
