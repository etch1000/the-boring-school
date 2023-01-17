// @generated automatically by Diesel CLI.

diesel::table! {
    grades (grade_id) {
        grade_id -> Integer,
        student_id -> Integer,
        subject_name -> Text,
        assignment_score -> Integer,
        test_score -> Integer,
    }
}

diesel::table! {
    students (student_id) {
        student_id -> Integer,
        student_name -> Text,
        class_id -> Integer,
        contact_info -> Text,
        email -> Text,
    }
}

diesel::table! {
    subs (subject_name) {
        class_id -> Integer,
        subject_name -> Text,
        teacher_id -> Integer,
    }
}

diesel::table! {
    teachers (teacher_id) {
        teacher_id -> Integer,
        teacher_name -> Text,
        subject_name -> Text,
        email -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(grades, students, subs, teachers,);
