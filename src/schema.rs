// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Integer,
        name -> Text,
        tutor -> Nullable<Text>,
    }
}

diesel::table! {
    historys (id) {
        id -> Integer,
        student_id -> Integer,
        course_id -> Integer,
        date -> Text,
        score -> Integer,
    }
}

diesel::table! {
    students (id) {
        id -> Integer,
        name -> Text,
        pw -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    historys,
    students,
);
