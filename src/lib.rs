use rocket::{get, routes, Build, Rocket};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

pub fn rocket_builder() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, hello])
}
