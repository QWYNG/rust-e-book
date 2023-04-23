use crate::schema::students;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub pw: Option<String>,
}
