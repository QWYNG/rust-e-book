use crate::course::Course;
use crate::schema::histories;
use crate::student::Student;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Identifiable, Selectable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug,
)]
#[diesel(belongs_to(Course, foreign_key = course_id))]
#[diesel(belongs_to(Student, foreign_key = student_id))]
#[diesel(table_name = histories)]
pub struct History {
    pub id: i32,
    pub course_id: i32,
    pub student_id: i32,
    pub date: String,
    pub score: i32,
}
