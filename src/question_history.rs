use crate::{
    question::Question,
    schema::{question_histories, questions, students},
    student::Student,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

joinable!(question_histories -> students(student_id));
joinable!(question_histories -> questions(question_id));

#[derive(
    Identifiable, Selectable, Queryable, Serialize, Deserialize, Associations, PartialEq, Debug,
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
