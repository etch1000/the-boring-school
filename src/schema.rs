// @generated automatically by Diesel CLI.

diesel::table! {
    students (roll_number) {
        name -> Text,
        roll_number -> Integer,
        class -> Integer,
        english_score -> Integer,
        computer_score -> Integer,
        math_score -> Integer,
        sports_score -> Integer,
    }
}

diesel::table! {
    teachers (name) {
        name -> Text,
        class -> Integer,
        course -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(students, teachers,);
