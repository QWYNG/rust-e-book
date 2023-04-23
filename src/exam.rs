use crate::schema::exams;
use crate::schema::courses;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

joinable!(exams -> courses(course_id));

#[derive(Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = exams)]
#[diesel(belongs_to(Course, foreign_key = course_id))]
pub struct Exam {
    pub id: i32,
    pub course_id: i32,
    pub name: String,
}
