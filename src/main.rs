#[macro_use]
extern crate rocket;
pub mod error;
pub mod insulin;
pub mod routes;
use routes::get_routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", get_routes())
}
