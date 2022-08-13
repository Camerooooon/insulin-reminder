#[macro_use]
extern crate rocket;
mod insulin;
mod routes;
use insulin::parse_dose;
use routes::get_routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", get_routes())
}
