use rocket::http::Status;
mod common;

#[test]
fn hello_world() {
    let client = common::setup();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}

#[test]
fn hello_name() {
    let client = common::setup();
    let response = client.get("/name").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, name!");
}
