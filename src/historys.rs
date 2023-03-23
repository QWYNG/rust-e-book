use crate::courses::Course;
use crate::schema::historys;
use crate::students::Student;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Course))]
#[diesel(belongs_to(Student))]
#[diesel(table_name = historys)]
pub struct Historys {
    pub id: i32,
    course_id: i32,
    student_id: i32,
    pub date: String,
    pub score: i32,
}
