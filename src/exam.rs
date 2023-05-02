use crate::course::Course;
use crate::schema::exams;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Identifiable, Selectable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug,
)]
#[diesel(table_name = exams)]
#[diesel(belongs_to(Course, foreign_key = course_id))]
pub struct Exam {
    pub id: i32,
    pub course_id: i32,
    pub name: String,
}
