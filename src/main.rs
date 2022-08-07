#[macro_use] extern crate rocket;
mod insulin;
use insulin::{get_lines, parse_dose};

#[get("/")]
fn index() -> String {
    parse_dose();
    "Yo".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
