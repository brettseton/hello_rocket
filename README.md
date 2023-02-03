# RUST: Hello Rocket

This is just a simple tutorial on getting unit test and benchmarks setup using Rocket in Rust.

To run app:

    cargo run

To run unit tests:

    cargo test

To run benchmarks:

    cargo bench

## Docker

To build in Docker:

    Docker build -f Dockerfile -t hello_rocket:v0.1 .
To run in Docker:

    Docker run --rm -d -p 8000:8000 hello_rocket:v0.1