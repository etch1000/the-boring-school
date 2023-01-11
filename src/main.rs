mod models;
mod schema;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use models::*;
use rocket::response::status;
use rocket::serde::json::Json;
use schema::*;
use the_boring_school::establish_connection;

#[get("/")]
fn home_page() -> &'static str {
    "Welcome To The Boring School"
}

// ACCESSIBLE TO: Principal & Teacher
#[post("/add_student", format = "json", data = "<new_student>")]
async fn add_student(
    new_student: Json<Student>,
) -> Result<status::Created<String>, status::Unauthorized<String>> {
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

// ACCESSIBLE TO: Principal
#[post("/add_teacher", format = "json", data = "<new_teacher>")]
async fn add_teacher(
    new_teacher: Json<Teacher>,
) -> Result<status::Created<String>, status::Unauthorized<String>> {
    let c = establish_connection();

    let res = diesel::insert_into(teachers::table)
        .values(new_teacher.into_inner())
        .execute(&c);

    if res == Ok(1) {
        Ok(status::Created::new("Teacher added Successfully"))
    } else {
        Err(status::Unauthorized(Some(String::from(
            "You are not allowed to do that action without proper auth token",
        ))))
    }
}

// ACCESSIBLE TO: ALL
#[get("/all_teachers")]
async fn get_all_teachers() -> Json<Vec<Teacher>> {
    let c = establish_connection();
    Json(teachers::table.load::<Teacher>(&c).unwrap())
}

// ACCESSIBLE TO: Principal & Teacher
#[get("/all_students")]
async fn get_all_student() -> Json<Vec<(String, i32, i32)>> {
    let c = establish_connection();
    Json(
        students::table
            .select((students::name, students::class, students::roll_number))
            .load::<(String, i32, i32)>(&c)
            .unwrap(),
    )
}


pub enum Role {
    Principal,
    Teacher,
    Student,
}

pub struct Claim {
    pub id: Role,
    pub sub: String,
    pub exp: u128,
}
// ACCESSIBLE TO: ALL
#[get("/?<class>&<roll_number>")]
async fn get_result(class: i32, roll_number: i32) -> Json<Option<Student>> {
    let c = establish_connection();
    Json(
        students::table
            .filter(students::class.eq(class))
            .filter(students::roll_number.eq(roll_number))
            .first::<Student>(&c)
            .ok(),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(School::fairing())
        .mount(
            "/",
            routes![
                home_page,
                add_student,
                add_teacher,
                get_all_student,
                get_all_teachers
            ],
        )
        .mount("/result", routes![get_result])
}
