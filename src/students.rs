use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub pw: Option<String>
}
