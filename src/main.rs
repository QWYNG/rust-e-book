#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;

use rocket_dyn_templates::{Template, context};
use std::cmp::Ordering;

static SECRET_NUMBER: i32 = 42;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { message: "Please input your guess number" })
}

#[get("/?<number>")]
fn index_number(number: i32) -> Template {

    let message = match number.cmp(&SECRET_NUMBER) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => {
            "You win!"
        }
    };


    Template::render("index_number", context! { message: message, guessed_number: number})
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, index_number])
        .attach(Template::fairing())
}