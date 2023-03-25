use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::courses;

#[derive(Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub tutor: Option<String>,
}
