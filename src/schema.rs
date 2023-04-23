// @generated automatically by Diesel CLI.

diesel::table! {
    chapters (id) {
        id -> Integer,
        course_id -> Integer,
        name -> Text,
        content -> Text,
    }
}

diesel::table! {
    courses (id) {
        id -> Integer,
        name -> Text,
        tutor -> Nullable<Text>,
    }
}

diesel::table! {
    exams (id) {
        id -> Integer,
        course_id -> Integer,
        name -> Text,
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
    mentors (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    questions (id) {
        id -> Integer,
        exam_id -> Integer,
        content -> Text,
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
    chapters,
    courses,
    exams,
    historys,
    mentors,
    questions,
    students,
);
