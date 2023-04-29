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

mod chapter;
mod course;
mod exam;
mod exam__history;
mod history;
mod mentor;
mod question;
mod question_history;
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

#[get("/<student_name>/<course_id>/show")]
async fn course_show(student_name: String, course_id: i32) -> Template {
    use self::schema::students::dsl::*;

    use self::chapter::Chapter;
    use self::course::Course;
    use self::exam::Exam;
    use self::student::Student;

    let conn = &mut establish_connection_sqlite();

    let student = students
        .filter(self::schema::students::dsl::name.eq(student_name))
        .first::<Student>(conn)
        .expect("error loading student");

    let course = self::schema::courses::table
        .filter(self::schema::courses::dsl::id.eq(course_id))
        .first::<Course>(conn)
        .expect("error loading course");
    let chapters = Chapter::belonging_to(&course)
        .select(Chapter::as_select())
        .load::<Chapter>(conn)
        .expect("error loading chapters");
    let exams = Exam::belonging_to(&course)
        .select(exam::Exam::as_select())
        .load::<exam::Exam>(conn)
        .expect("error loading exams");

    Template::render(
        "course/show",
        context! { student: &student, course: &course, chapters: &chapters, exams: &exams},
    )
}

#[get("/<student_name>/<course_id>/<exam_id>/show")]
async fn exam_show(student_name: String, course_id: i32, exam_id: i32) -> Template {
    use self::schema::students::dsl::*;

    use self::course::Course;
    use self::exam::Exam;
    use self::question::Question;
    use self::student::Student;

    let conn = &mut establish_connection_sqlite();

    let student = students
        .filter(self::schema::students::dsl::name.eq(student_name))
        .first::<Student>(conn)
        .expect("error loading student");

    let course = self::schema::courses::table
        .filter(self::schema::courses::dsl::id.eq(course_id))
        .first::<Course>(conn)
        .expect("error loading course");
    let exam = self::schema::exams::table
        .filter(self::schema::exams::dsl::id.eq(exam_id))
        .first::<Exam>(conn)
        .expect("error loading exam");
    let questions = Question::belonging_to(&exam)
        .select(Question::as_select())
        .load::<Question>(conn)
        .expect("error loading questions");

    Template::render(
        "exam/show",
        context! { student: &student, course: &course, exam: &exam, questions: &questions},
    )
}

#[get("/<mentor_name>/mentor_index")]
async fn mentor_index(mentor_name: String) -> Template {
    use self::schema::mentors::dsl::*;

    use self::mentor::Mentor;
    use self::student::Student;

    let conn = &mut establish_connection_sqlite();

    let mentor = mentors
        .filter(self::schema::mentors::dsl::name.eq(mentor_name))
        .first::<Mentor>(conn)
        .expect("error loading mentor");
    let students = self::schema::students::table
        .select(Student::as_select())
        .load::<Student>(conn)
        .expect("error");

    Template::render(
        "mentor/index",
        context! { mentor: &mentor, students: students },
    )
}

#[get("/<student_name>/not_complete_courses")]
async fn not_complete_courses(student_name: String) -> Template {
    use self::course::Course;
    use self::schema::students::dsl::*;
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

#[post("/mentor_login", data = "<user_form>")]
async fn post_mentor_login(user_form: Form<UserForm>) -> Redirect {
    use self::mentor::Mentor;
    use self::schema::mentors::dsl::*;
    let user_name = &user_form.name;

    let conn = &mut establish_connection_sqlite();
    let mentor = mentors
        .filter(name.eq(user_name))
        .first::<Mentor>(conn)
        .expect("Error loading mentors");

    Redirect::to(uri!(_, mentor_index(mentor.name)))
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing()).mount(
        "/",
        routes![
            login,
            post_login,
            post_mentor_login,
            course_show,
            exam_show,
            index,
            mentor_index,
            not_complete_courses,
            create_history
        ],
    )
}
