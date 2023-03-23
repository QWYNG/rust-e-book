#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rocket_contrib;
extern crate rocket_dyn_templates;

use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::{form::Form, response::Redirect};
use rocket_dyn_templates::{context, Template};

mod courses;
mod historys;
mod schema;
mod students;

use std::env;

pub fn establish_connection_sqlite() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/index/<student_name>")]
async fn index(student_name: String) -> Template {
    use self::schema::courses::dsl::*;
    use self::schema::students::dsl::*;

    use self::courses::Course;
    use self::students::Student;

    let conn = &mut establish_connection_sqlite();

    let courses_results = courses.load::<Course>(conn).expect("Error loading courses");

    let student = students
        .filter(self::schema::students::dsl::name.eq(student_name))
        .first::<Student>(conn)
        .expect("error loading student");

    Template::render(
        "index",
        context! { student: &student, courses: &courses_results},
    )
}

#[get("/login")]
async fn login() -> Template {
    Template::render("login", context! {})
}

#[derive(FromForm)]
struct UserForm {
    name: String,
}

#[post("/login", data = "<user_form>")]
async fn post_login(user_form: Form<UserForm>) -> Redirect {
    use self::schema::students::dsl::*;
    use self::students::Student;
    let user_name = &user_form.name;

    let conn = &mut establish_connection_sqlite();
    let student = students
        .filter(name.eq(user_name))
        .first::<Student>(conn)
        .expect("Error loading students");

    Redirect::to(uri!(_, index(student.name)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![login, post_login, index])
}
