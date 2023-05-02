use crate::exam::Exam;
use crate::schema::questions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Selectable, Queryable, Serialize, Deserialize, Associations)]
#[diesel(table_name = questions)]
#[diesel(belongs_to(Exam, foreign_key = exam_id))]
pub struct Question {
    pub id: i32,
    pub exam_id: i32,
    pub content: String,
}
