use hello_rocket::rocket_builder;
use rocket::local::blocking::Client;

pub fn setup() -> Client {
    Client::tracked(rocket_builder()).expect("valid rocket instance")
}