use crate::course::Course;
use crate::schema::{courses, historys, students};
use crate::student::Student;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

joinable!(historys -> students(student_id));
joinable!(historys -> courses(course_id));

#[derive(
    Identifiable, Selectable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug,
)]
#[diesel(belongs_to(Course, foreign_key = course_id))]
#[diesel(belongs_to(Student, foreign_key = student_id))]
#[diesel(table_name = historys)]
pub struct History {
    pub id: i32,
    pub course_id: i32,
    pub student_id: i32,
    pub date: String,
    pub score: i32,
}
