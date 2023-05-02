use crate::{exam::Exam, schema::exam_histories, student::Student};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Identifiable,
    Selectable,
    Queryable,
    Serialize,
    Deserialize,
    Associations,
    PartialEq,
    Debug,
    AsChangeset,
)]
#[diesel(belongs_to(Exam, foreign_key = exam_id))]
#[diesel(belongs_to(Student, foreign_key = student_id))]
#[diesel(table_name = exam_histories)]
pub struct ExamHistory {
    pub id: i32,
    pub exam_id: i32,
    pub student_id: i32,
    pub start_datetime: String,
    pub end_datetime: Option<String>,
    pub score: Option<i32>,
}
