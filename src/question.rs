use crate::schema::questions;
use crate::schema::exams;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

joinable!(questions -> exams(exam_id));

#[derive(Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = questions)]
#[diesel(belongs_to(Exam, foreign_key = exam_id))]
pub struct Question {
    pub id: i32,
    pub exam_id: i32,
    pub content: String,
}