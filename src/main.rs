#[macro_use]
extern crate rocket;
extern crate core;
extern crate diesel;
extern crate rocket_contrib;
extern crate rocket_dyn_templates;

use diesel::prelude::*;
use dotenvy::dotenv;
use history::History;
use rocket::{form::Form, response::Redirect};
use rocket_dyn_templates::{context, Template};

mod course;
mod history;
mod schema;
mod student;

use std::env;

pub fn establish_connection_sqlite() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/<student_name>/index")]
async fn index(student_name: String) -> Template {
    use self::schema::students::dsl::*;

    use self::course::Course;
    use self::student::Student;

    let conn = &mut establish_connection_sqlite();

    let student = students
        .filter(self::schema::students::dsl::name.eq(student_name))
        .first::<Student>(conn)
        .expect("error loading student");

    let historys_with_course: Vec<(History, Course)> = History::belonging_to(&student)
        .inner_join(self::schema::courses::table)
        .select((History::as_select(), Course::as_select()))
        .load(conn)
        .expect("error");

    Template::render(
        "index",
        context! { student: &student, historys_with_course: historys_with_course},
    )
}

#[get("/<student_name>/not_complete_courses")]
async fn not_complete_courses(student_name: String) -> Template {
    use self::schema::students::dsl::*;
    use self::course::Course;
    use self::student::Student;

    let conn = &mut establish_connection_sqlite();

    let student = students
        .filter(self::schema::students::dsl::name.eq(student_name))
        .first::<Student>(conn)
        .expect("error loading student");

    let not_complete_courses = self::schema::courses::table
        .left_join(self::schema::historys::table)
        .filter(self::schema::historys::id.is_null())
        .select(Course::as_select())
        .load::<Course>(conn)
        .expect("error");

    Template::render(
        "not_complete_courses",
        context! { student: &student, not_complete_courses: not_complete_courses},
    )
}

#[derive(FromForm)]
struct HistoryForm {
    date: String,
    score: i32,
}

#[post("/<student_name>/<course_id>", data = "<history_form>")]
async fn create_history(
    student_name: String,
    course_id: i32,
    history_form: Form<HistoryForm>,
) -> Redirect {
    use self::schema::courses::dsl::*;
    use self::schema::students::dsl::*;

    use self::course::Course;
    use self::student::Student;

    let conn = &mut establish_connection_sqlite();
    let student = students
        .filter(self::schema::students::dsl::name.eq(student_name))
        .first::<Student>(conn)
        .expect("error loading student");

    let course = courses
        .filter(self::schema::courses::dsl::id.eq(course_id))
        .first::<Course>(conn)
        .expect("error loading course");

    diesel::insert_into(self::schema::historys::table)
        .values((
            self::schema::historys::dsl::course_id.eq(&course.id),
            self::schema::historys::dsl::student_id.eq(&student.id),
            self::schema::historys::dsl::date.eq(&history_form.date),
            self::schema::historys::dsl::score.eq(&history_form.score),
        ))
        .execute(conn)
        .expect("error insert");

    Redirect::to(uri!(_, index(student.name)))
}

#[get("/")]
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
    use self::student::Student;
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
    rocket::build().attach(Template::fairing()).mount(
        "/",
        routes![
            login,
            post_login,
            index,
            not_complete_courses,
            create_history
        ],
    )
}
