#[macro_use]
extern crate rocket;
extern crate core;
extern crate diesel;
extern crate rocket_contrib;
extern crate rocket_dyn_templates;

use std::env;

use chrono::prelude::*;
use diesel::prelude::*;
use dotenvy::dotenv;
use history::History;
use rocket::{form::Form, response::Redirect};
use rocket_dyn_templates::{context, Template};

mod chapter;
mod course;
mod exam;
mod exam_history;
mod history;
mod mentor;
mod question;
mod question_history;
mod schema;
mod student;

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

    let histories_with_course: Vec<(History, Course)> = History::belonging_to(&student)
        .inner_join(self::schema::courses::table)
        .select((History::as_select(), Course::as_select()))
        .load(conn)
        .expect("error");

    Template::render(
        "index",
        context! { student: &student, histories_with_course: histories_with_course},
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
    let exams_with_history: Vec<(Exam, Option<String>, Option<String>, Option<i32>)> =
        self::schema::exams::table
            .left_join(self::schema::exam_histories::table)
            .filter(self::schema::exams::dsl::course_id.eq(course_id))
            .filter(
                self::schema::exam_histories::dsl::student_id
                    .eq(student.id)
                    .or(self::schema::exam_histories::dsl::student_id.is_null()),
            )
            .select((
                Exam::as_select(),
                self::schema::exam_histories::start_datetime.nullable(),
                self::schema::exam_histories::end_datetime.nullable(),
                self::schema::exam_histories::score.nullable(),
            ))
            .load(conn)
            .expect("error loading exams");

    Template::render(
        "course/show",
        context! { student: &student, course: &course, chapters: &chapters, exams_with_history: &exams_with_history},
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

    diesel::insert_into(self::schema::exam_histories::table)
        .values((
            self::schema::exam_histories::exam_id.eq(&exam.id),
            self::schema::exam_histories::student_id.eq(&student.id),
            self::schema::exam_histories::start_datetime.eq(Utc::now().to_string()),
        ))
        .on_conflict((
            self::schema::exam_histories::exam_id,
            self::schema::exam_histories::student_id,
        ))
        .do_nothing()
        .execute(conn)
        .expect("error insert");

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
        .left_join(self::schema::histories::table)
        .filter(self::schema::histories::id.is_null())
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

    diesel::insert_into(self::schema::histories::table)
        .values((
            self::schema::histories::dsl::course_id.eq(&course.id),
            self::schema::histories::dsl::student_id.eq(&student.id),
            self::schema::histories::dsl::date.eq(&history_form.date),
            self::schema::histories::dsl::score.eq(&history_form.score),
        ))
        .execute(conn)
        .expect("error insert");

    Redirect::to(uri!(_, index(student.name)))
}

#[derive(FromForm)]
struct QuestionAnswerForm {
    correct: bool,
}

#[post(
    "/<student_name>/<course_id>/<exam_id>/<question_id>/answer",
    data = "<question_answer_form>"
)]
async fn answer_question(
    student_name: String,
    course_id: i32,
    exam_id: i32,
    question_id: i32,
    question_answer_form: Form<QuestionAnswerForm>,
) -> Redirect {
    use self::schema::students::dsl::*;

    use self::exam::Exam;
    use self::exam_history::ExamHistory;
    use self::question::Question;
    use self::question_history::QuestionHistory;
    use self::student::Student;

    let conn = &mut establish_connection_sqlite();
    let student = students
        .filter(self::schema::students::dsl::name.eq(student_name))
        .first::<Student>(conn)
        .expect("error loading student");

    let exam = self::schema::exams::dsl::exams
        .filter(self::schema::exams::dsl::id.eq(exam_id))
        .first::<Exam>(conn)
        .expect("error loading exam");

    let question = self::schema::questions::dsl::questions
        .filter(self::schema::questions::dsl::id.eq(question_id))
        .first::<Question>(conn)
        .expect("error loading question");

    diesel::insert_into(self::schema::question_histories::dsl::question_histories)
        .values((
            self::schema::question_histories::question_id.eq(&question.id),
            self::schema::question_histories::student_id.eq(&student.id),
            self::schema::question_histories::correct.eq(&question_answer_form.correct),
        ))
        .on_conflict((
            self::schema::question_histories::question_id,
            self::schema::question_histories::student_id,
        ))
        .do_update()
        .set(self::schema::question_histories::correct.eq(&question_answer_form.correct))
        .execute(conn)
        .expect("error insert");

    let questions_result = self::schema::questions::table
        .filter(self::schema::questions::exam_id.eq(&exam.id))
        .load::<Question>(conn)
        .expect("error loading questions");

    let question_histories_result = self::schema::question_histories::table
        .filter(self::schema::question_histories::student_id.eq(&student.id))
        .filter(
            self::schema::question_histories::question_id.eq_any(
                questions_result
                    .iter()
                    .map(|question_result| question_result.id)
                    .collect::<Vec<i32>>(),
            ),
        )
        .load::<QuestionHistory>(conn)
        .expect("error loading question histories");

    if question_histories_result.len() == questions_result.len() {
        let exam_history_result = self::schema::exam_histories::table
            .filter(self::schema::exam_histories::student_id.eq(&student.id))
            .filter(self::schema::exam_histories::exam_id.eq(&exam.id))
            .first::<ExamHistory>(conn)
            .expect("error loading exam histories");

        diesel::update(&exam_history_result)
            .set((
                self::schema::exam_histories::score.eq(question_histories_result
                    .iter()
                    .filter(|question_history_result| question_history_result.correct)
                    .count() as i32),
                self::schema::exam_histories::end_datetime.eq(Utc::now().to_string()),
            ))
            .execute(conn)
            .expect("error insert");
    }
    Redirect::to(uri!(_, exam_show(student.name, course_id, exam_id)))
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
            create_history,
            answer_question
        ],
    )
}
