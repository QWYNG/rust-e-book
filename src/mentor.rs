use crate::schema::mentors;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = mentors)]
pub struct Mentor {
    pub id: i32,
    pub name: String
}
