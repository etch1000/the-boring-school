// @generated automatically by Diesel CLI.

diesel::table! {
    grades (grade_id) {
        grade_id -> Integer,
        student_id -> Integer,
        class_name -> Text,
        assignment_score -> Integer,
        test_score -> Integer,
    }
}

diesel::table! {
    students (student_id) {
        student_id -> Integer,
        student_name -> Text,
        contact_info -> Text,
        email -> Text,
    }
}

diesel::table! {
    subs (class_id) {
        class_id -> Integer,
        class_name -> Text,
        teacher_id -> Nullable<Integer>,
    }
}

diesel::table! {
    teachers (teacher_id) {
        teacher_id -> Integer,
        teacher_name -> Text,
        class_name -> Text,
        email -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(grades, students, subs, teachers,);
