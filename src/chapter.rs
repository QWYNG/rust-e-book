use crate::course::Course;
use crate::schema::{courses, chapters};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

joinable!(chapters -> courses(course_id));


#[derive(
    Identifiable, Selectable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug,
)]
#[diesel(belongs_to(Course, foreign_key = course_id))]
#[diesel(table_name = chapters)]
pub struct Chapter {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub course_id: i32,
}
