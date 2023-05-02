use crate::{question::Question, schema::question_histories, student::Student};
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
#[diesel(belongs_to(Question, foreign_key = question_id))]
#[diesel(belongs_to(Student, foreign_key = student_id))]
#[diesel(table_name = question_histories)]
pub struct QuestionHistory {
    pub id: i32,
    pub question_id: i32,
    pub student_id: i32,
    pub correct: bool,
}
