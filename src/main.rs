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
use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket::{response::status, serde::json::Json};
use rocket_okapi::swagger_ui::make_swagger_ui;
use rocket_okapi::{openapi, openapi_get_routes};
use schema::*;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use swagger::swag_config;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

#[openapi(tag = "Home")]
#[get("/")]
fn home_page(_auth: Claims) -> &'static str {
    "Welcome To The Boring School"
}

// ACCESSIBLE TO: Principal & Teacher
#[openapi(tag = "AddOp")]
#[post("/add_subject", format = "json", data = "<new_subject>")]
async fn add_subject(
    auth: Claims,
    new_subject: Json<Sub>,
) -> Result<status::Created<String>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            let c = establish_connection();
            println!("Hello, world!");
            let res = diesel::insert_into(subs::table)
                .values(new_subject.into_inner())
                .execute(&c);
            if res == Ok(1) {
                Ok(status::Created::new("Subject added successfully"))
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

// ACCESSIBLE TO: Principal & Teacher
#[openapi(tag = "AddOp")]
#[post("/add_student", format = "json", data = "<new_student>")]
async fn add_student(
    auth: Claims,
    new_student: Json<Student>,
) -> Result<status::Created<String>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            println!("Hello, world!");
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
                    "You are not allowed to do that",
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

#[openapi(tag = "GetOp")]
#[get("/all_teachers_of_class/<class_id>")]
async fn get_teachers_of_class(class_id: i32) -> Json<Vec<Teacher>> {
    let c = establish_connection();
    let mut res: Vec<Teacher> = vec![];
    let class = subs::table
        .filter(subs::class_id.eq(class_id))
        .load::<Sub>(&c)
        .unwrap();
    for (_, sub) in class.iter().enumerate() {
        let teacher = teachers::table
            .filter(teachers::teacher_id.eq(sub.teacher_id))
            .first::<Teacher>(&c)
            .unwrap();
        res.push(teacher)
    }
    Json(res)
}

// ACCESSIBLE TO: ALL
#[openapi(tag = "GetOp")]
#[get("/teacher/<class_id>/<subject_name>")]
async fn get_class_sub_teacher(class_id: i32, subject_name: String) -> Json<Teacher> {
    let c = establish_connection();
    let teacher_id = subs::table
        .filter(subs::class_id.eq(class_id))
        .filter(subs::subject_name.eq(subject_name))
        .first::<Sub>(&c)
        .unwrap()
        .teacher_id;
    let teacher = teachers::table
        .filter(teachers::teacher_id.eq(teacher_id))
        .first(&c)
        .unwrap();
    Json(teacher)
}

// ACCESSIBLE TO: Principal & Teacher
#[openapi(tag = "GetOp")]
#[get("/all_students")]
async fn get_all_student(auth: Claims) -> Result<Json<Vec<Student>>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            let c = establish_connection();

            println!("{:#?}", students::table.load::<Student>(&c).unwrap());

            Ok(Json(students::table.load(&c).unwrap()))
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

// ACCESSIBLE TO: PRINCIPAL, TEACHERS AND THE STUDENT WITH THE PASSED student_id
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

#[openapi(tag = "UpdateOp")]
#[patch(
    "/update_assignment_score/<student_name>/<subject_name>",
    format = "json",
    data = "<new_assignment_score>"
)]
async fn update_assignment_score(
    auth: Claims,
    student_name: String,
    subject_name: String,
    new_assignment_score: Json<i32>,
) -> Result<status::Custom<String>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            let c = establish_connection();
            let student_id = students::table
                .filter(students::student_name.eq(student_name))
                .first::<Student>(&c)
                .unwrap()
                .student_id;
            let res = diesel::update(grades::table)
                .filter(grades::student_id.eq(student_id))
                .filter(grades::subject_name.eq(subject_name))
                .set(grades::assignment_score.eq(new_assignment_score.into_inner()))
                .execute(&c)
                .unwrap();
            match res {
                1 => Ok(status::Custom(
                    Status::Ok,
                    String::from("Assignment score updated successfully"),
                )),
                _ => Ok(status::Custom(
                    Status::NotModified,
                    String::from("Something went wrong and the assignment score is unchanged"),
                )),
            }
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are note allowed to do that",
        )))),
    }
}

#[openapi(tag = "UpdateOp")]
#[patch(
    "/update_student_name/<student_name>",
    format = "json",
    data = "<new_student_name>"
)]
async fn update_student_name(
    auth: Claims,
    student_name: String,
    new_student_name: Json<String>,
) -> Result<status::Custom<String>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            let c = establish_connection();
            let res = diesel::update(students::table)
                .filter(students::student_name.eq(student_name))
                .set(students::student_name.eq(new_student_name.into_inner()))
                .execute(&c)
                .unwrap();
            match res {
                1 => Ok(status::Custom(
                    Status::Ok,
                    String::from("Student name updated successfully"),
                )),
                _ => Ok(status::Custom(
                    Status::NotModified,
                    String::from("Something went wrong and we have not updated the student name"),
                )),
            }
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are not allowed to do that",
        )))),
    }
}

// ACCESSIBLE TO: Principal
#[openapi(tag = "UpdateOp")]
#[patch("/update_teacher_email/<teacher_name>", format = "json", data = "<new_email_address>")]
async fn update_teacher_email(auth: Claims, teacher_name: String, new_email_address: Json<String>) -> Result<status::Custom<String>, status::Unauthorized<String>> {
    match auth.id {
        3 => {
            let c = establish_connection();
            let res = diesel::update(teachers::table).filter(teachers::teacher_name.eq(teacher_name)).set(teachers::email.eq(new_email_address.into_inner())).execute(&c).unwrap();
            match res {
                1 => Ok(status::Custom(Status::Ok, String::from("Teacher's email id was successfully updated"))),
                _ => Ok(status::Custom(Status::NotModified, String::from("Something went wrong and the teacher's email was not changed")))
            }
        }
        _ => Err(status::Unauthorized(Some(String::from("You are not allowed to do that"))))
    }
}

#[openapi(tag = "DeleteOp")]
#[delete("/student/<student_id>")]
async fn remove_student(
    auth: Claims,
    student_id: i32,
) -> Result<status::Accepted<String>, status::Unauthorized<String>> {
    match auth.id {
        3 => {
            let c = establish_connection();
            let res = diesel::delete(students::table.filter(students::student_id.eq(student_id)))
                .execute(&c)
                .unwrap();
            match res {
                1 => Ok(status::Accepted(Some(String::from(
                    "Student removed from the records",
                )))),
                _ => Err(status::Unauthorized(Some(String::from(
                    "You are not allowed to do that",
                )))),
            }
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are not allowed to do that",
        )))),
    }
}

#[openapi(tag = "DeleteOp")]
#[delete("/teacher/<teacher_id>")]
async fn remove_teacher(
    auth: Claims,
    teacher_id: i32,
) -> Result<status::Accepted<String>, status::Unauthorized<String>> {
    match auth.id {
        3 => {
            let c = establish_connection();
            let res = diesel::delete(teachers::table.filter(teachers::teacher_id.eq(teacher_id)))
                .execute(&c)
                .unwrap();
            match res {
                1 => Ok(status::Accepted(Some(String::from(
                    "Teacher was removed from the records",
                )))),
                _ => Err(status::Unauthorized(Some(String::from(
                    "You are not allowed to do that",
                )))),
            }
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are not allowed to do that",
        )))),
    }
}

#[openapi(tag = "DeleteOp")]
#[delete("/grade/<student_name>/<subject_name>")]
fn remove_grade(
    auth: Claims,
    student_name: String,
    subject_name: String,
) -> Result<status::Accepted<String>, status::Unauthorized<String>> {
    match auth.id {
        3 | 2 => {
            let c = establish_connection();
            let student_id = students::table
                .filter(students::student_name.eq(student_name))
                .first::<Student>(&c)
                .unwrap()
                .student_id;
            let grade_id = grades::table
                .filter(grades::subject_name.eq(subject_name))
                .filter(grades::student_id.eq(student_id))
                .first::<Grade>(&c)
                .unwrap()
                .grade_id;

            let res = diesel::delete(grades::table.filter(grades::grade_id.eq(grade_id)))
                .execute(&c)
                .unwrap();

            match res {
                1 => Ok(status::Accepted(Some(String::from(
                    "Grade record was removed",
                )))),
                _ => Err(status::Unauthorized(Some(String::from(
                    "You are not allowed to do that",
                )))),
            }
        }
        _ => Err(status::Unauthorized(Some(String::from(
            "You are not allowed to do that",
        )))),
    }
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
                add_grade,
                add_student,
                add_subject,
                add_teacher,
                get_class_sub_teacher,
                get_all_student,
                get_all_teachers,
                get_teachers_of_class,
                get_student,
                get_grades,
                update_assignment_score,
                update_student_name,
                update_teacher_email,
                remove_student,
                remove_teacher,
                remove_grade,
            ],
        )
}

pub fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn get_client() -> Client {
    Client::tracked(rocket()).expect("Valid Rocket Instance")
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel::result::Error;
    use rocket::http::Header;
    use rocket::http::Status;

    #[test]
    fn test_homepage() {
        let tbsuser = Claims {
            id: 3,
            iat: current_time(),
            aud: String::from("TBS"),
            sub: String::from("TBSUSER"),
            exp: current_time() + 2000,
        };

        let jwtoken = auth::_encode_jwt(tbsuser).unwrap();

        let client = get_client();

        let res = client
            .get(uri!(home_page))
            .header(Header::new("Authorization", format!("Bearer {jwtoken}")))
            .dispatch();

        assert_eq!(Status::Ok, res.status());
    }

    #[test]
    fn test_add_student() {
        let rancho = Student {
            student_id: 1000,
            student_name: String::from("Rancho"),
            class_id: 1,
            contact_info: String::from("8770207535"),
            email: String::from("rancho@hogwarts.com"),
        };

        let c = establish_connection();

        c.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(students::table)
                .values(rancho)
                .execute(&c)?;

            let all_students_names = students::table
                .select(students::student_name)
                .load::<String>(&c)?;

            assert_eq!(
                vec![
                    "Harry Potter",
                    "Ronald Weasley",
                    "Hermione Granger",
                    "Rancho"
                ],
                all_students_names
            );

            Ok(())
        });

        let all_students_names_without_rancho = students::table
            .select(students::student_name)
            .load::<String>(&c)
            .unwrap();

        assert_eq!(
            vec!["Harry Potter", "Ronald Weasley", "Hermione Granger"],
            all_students_names_without_rancho
        );
    }

    #[test]
    fn test_add_subject() {
        let c = establish_connection();

        let new_subject = Sub {
            class_id: 1,
            subject_name: String::from("Curses"),
            teacher_id: 1,
        };

        c.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(subs::table)
                .values(new_subject)
                .execute(&c)?;

            let all_subs_names = subs::table.select(subs::subject_name).load::<String>(&c)?;

            assert_eq!(vec!["Curses", "Fly Broom", "Potions"], all_subs_names);

            Ok(())
        });

        let all_subs = subs::table
            .select(subs::subject_name)
            .load::<String>(&c)
            .unwrap();

        assert_eq!(vec!["Fly Broom", "Potions"], all_subs);
    }

    #[test]
    fn test_add_teacher() {
        let c = establish_connection();

        let new_teacher = Teacher {
            teacher_id: 3,
            teacher_name: String::from("Rancho"),
            subject_name: String::from("Macines"),
            email: String::from("rancho@hogwarts.com"),
        };

        c.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(teachers::table)
                .values(new_teacher)
                .execute(&c)?;

            let all_teachers_names = teachers::table
                .select(teachers::teacher_name)
                .load::<String>(&c)?;

            assert_eq!(
                vec!["Severous Snape", "Rolanda Hooch", "Rancho"],
                all_teachers_names
            );

            Ok(())
        });

        let all_teachers = teachers::table
            .select(teachers::teacher_name)
            .load::<String>(&c)
            .unwrap();

        assert_eq!(vec!["Severous Snape", "Rolanda Hooch"], all_teachers);
    }

    #[test]
    fn test_add_grade() {
        let c = establish_connection();

        let rancho_grade = Grade {
            grade_id: 10,
            student_id: 1000,
            subject_name: String::from("Machines"),
            assignment_score: 10,
            test_score: 100,
        };

        c.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(grades::table)
                .values(rancho_grade)
                .execute(&c)?;

            let rancho_grade = grades::table
                .select(grades::subject_name)
                .filter(grades::student_id.eq(1000))
                // let res = client.get(uri!(home_page)).dispatch();
                .load::<String>(&c)?;

            assert_eq!(vec![String::from("Machines")], rancho_grade);

            Ok(())
        });

        let all_sub_test_scores = grades::table
            .select(grades::subject_name)
            .filter(grades::student_id.eq(1))
            .load::<String>(&c)
            .unwrap();

        assert_eq!(
            vec![String::from("Potions"), String::from("Fly Broom")],
            all_sub_test_scores
        );
    }

    #[test]
    fn test_get_all_students() {
        let c = get_client();

        use rocket::http::Header;
        let auth_user = Claims {
            id: 3,
            iat: current_time(),
            aud: String::from("TBS"),
            sub: String::from("TBSUSER"),
            exp: current_time() + 2000,
        };

        let jwtoken = auth::_encode_jwt(auth_user).unwrap();

        let expected_response = vec![
            Student {
                student_id: 1,
                student_name: String::from("Harry Potter"),
                class_id: 1,
                contact_info: String::from("8770207535"),
                email: String::from("harry_potter@hogwarts.com"),
            },
            Student {
                student_id: 2,
                student_name: String::from("Ronald Weasley"),
                class_id: 1,
                contact_info: String::from("9770207535"),
                email: String::from("ronald_weasley@hogwarts.com"),
            },
            Student {
                student_id: 3,
                student_name: String::from("Hermione Granger"),
                class_id: 1,
                contact_info: String::from("9770207536"),
                email: String::from("hermione_granger@hogwarts.com"),
            },
        ];

        let res = c
            .get(uri!(get_all_student))
            .header(Header::new("Authorization", format!("Bearer {jwtoken}")))
            .dispatch();

        assert_eq!(expected_response, res.into_json::<Vec<Student>>().unwrap());
    }

    #[test]
    fn test_get_all_teachers() {
        let c = get_client();

        let auth_user = Claims {
            id: 3,
            iat: current_time(),
            aud: String::from("TBS"),
            sub: String::from("TBSUSER"),
            exp: current_time() + 2000,
        };

        let expected_result = vec![
            Teacher {
                teacher_id: 1,
                teacher_name: String::from("Severous Snape"),
                subject_name: String::from("Potions"),
                email: String::from("halfbloodprince@hogwarts.com"),
            },
            Teacher {
                teacher_id: 2,
                teacher_name: String::from("Rolanda Hooch"),
                subject_name: String::from("Fly Broom"),
                email: String::from("rolandahooch@hogwarts.com"),
            },
        ];

        let jwtoken = auth::_encode_jwt(auth_user).unwrap();

        let res = c
            .get(uri!(super::get_all_teachers))
            .header(Header::new("Authorization", format!("Bearer {jwtoken}")))
            .dispatch();
        // let res = client.get(uri!(home_page)).dispatch();

        assert_eq!(expected_result, res.into_json::<Vec<Teacher>>().unwrap());
    }

    #[test]
    fn test_get_grades_for_harry() {
        let c = get_client();

        let auth_user = Claims {
            id: 3,
            iat: current_time(),
            aud: String::from("TBS"),
            sub: String::from("TBSUSER"),
            exp: current_time() + 2000,
        };

        let jwtoken = auth::_encode_jwt(auth_user).unwrap();

        let expected_result = vec![
            Grade {
                grade_id: 1,
                student_id: 1,
                subject_name: String::from("Potions"),
                assignment_score: 10,
                test_score: 100,
            },
            Grade {
                grade_id: 2,
                student_id: 1,
                subject_name: String::from("Fly Broom"),
                assignment_score: 10,
                test_score: 100,
            },
        ];

        let res = c
            .get("/grades/1")
            .header(Header::new("Authorization", format!("Bearer {jwtoken}")))
            .dispatch();

        assert_eq!(expected_result, res.into_json::<Vec<Grade>>().unwrap())
    }

    #[test]
    fn test_class_sub_teacher() {
        let c = get_client();
        let auth_user = Claims {
            id: 3,
            iat: current_time(),
            aud: String::from("TBS"),
            sub: String::from("TBSUSER"),
            exp: current_time() + 2000,
        };

        let jwtoken = auth::_encode_jwt(auth_user).unwrap();

        let res = c
            .get("/teacher/1/Potions")
            .header(Header::new("Authorization", format!("Bearer {jwtoken}")))
            .dispatch();

        let expected_response = Json(Teacher {
            teacher_id: 1,
            teacher_name: String::from("Severous Snape"),
            subject_name: String::from("Potions"),
            email: String::from("halfbloodprince@hogwarts.com"),
        });

        assert_eq!(
            expected_response.into_inner(),
            res.into_json::<Teacher>().unwrap()
        );
    }

    #[test]
    fn test_teachers_of_class_id() {
        let c = get_client();

        let auth_user = Claims {
            id: 3,
            iat: current_time(),
            aud: String::from("TBS"),
            sub: String::from("TBSUSER"),
            exp: current_time() + 2000,
        };

        let jwtoken = auth::_encode_jwt(auth_user).unwrap();

        let res = c
            .get("/all_teachers_of_class/1")
            .header(Header::new("Authorization", format!("Bearer {jwtoken}")))
            .dispatch();

        let expected_response = vec![
            Teacher {
                teacher_id: 1,
                teacher_name: String::from("Severous Snape"),
                subject_name: String::from("Potions"),
                email: String::from("halfbloodprince@hogwarts.com"),
            },
            Teacher {
                teacher_id: 2,
                teacher_name: String::from("Rolanda Hooch"),
                subject_name: String::from("Fly Broom"),
                email: String::from("rolandahooch@hogwarts.com"),
            },
        ];

        assert_eq!(expected_response, res.into_json::<Vec<Teacher>>().unwrap());
    }

    #[test]
    fn test_get_student_hermione() {
        let c = get_client();

        let auth_user = Claims {
            id: 3,
            iat: current_time(),
            aud: String::from("TBS"),
            sub: String::from("TBSUSER"),
            exp: current_time() + 2000,
        };

        let jwtoken = auth::_encode_jwt(auth_user).unwrap();

        let expected_result = Student {
            student_id: 3,
            student_name: String::from("Hermione Granger"),
            class_id: 1,
            contact_info: String::from("9770207536"),
            email: String::from("hermione_granger@hogwarts.com"),
        };

        let res = c
            .get("/student/3")
            .header(Header::new("Authorization", format!("Bearer {jwtoken}")))
            .dispatch();

        assert_eq!(expected_result, res.into_json::<Student>().unwrap());
    }
}
