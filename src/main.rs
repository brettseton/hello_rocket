#[macro_use]
extern crate rocket;

use hello_rocket::rocket_builder;
use rocket::{Build, Rocket};

#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket_builder()
}
