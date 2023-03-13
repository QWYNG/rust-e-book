#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate diesel;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::{Template, context};
use diesel::{prelude::*};
use dotenvy::dotenv;

pub mod schema;
mod students;

use std::env;


pub fn establish_connection_sqlite() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[get("/")]
async fn index() -> Template {
    use self::students::Student;

    let conn = &mut establish_connection_sqlite();
    let results = self::schema::students::dsl::students.load::<Student>(conn).expect("Error loading posts");
    Template::render("index", context! { students: &results})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}
