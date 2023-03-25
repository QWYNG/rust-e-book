use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::students;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub pw: Option<String>,
}
