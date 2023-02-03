use criterion::{criterion_group, criterion_main, Criterion};
use rocket::http::Status;

mod common;

fn hello_world_benchmark(c: &mut Criterion) {
    let client = common::setup();

    c.bench_function("hello world", |b| {
        b.iter(|| {
            let response = client.get("/").dispatch();
            assert_eq!(response.status(), Status::Ok);
            assert_eq!(response.into_string(), Some("Hello, world!".into()));
        })
    });
}

fn hello_name_benchmark(c: &mut Criterion) {
    let client = common::setup();

    c.bench_function("hello name", |b| {
        b.iter(|| {
            let response = client.get("/name").dispatch();
            assert_eq!(response.status(), Status::Ok);
            assert_eq!(response.into_string(), Some("Hello, name!".into()));
        })
    });
}

criterion_group!(benches, hello_world_benchmark, hello_name_benchmark);
criterion_main!(benches);
