FROM rust:latest as build
WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip --strip-all /app/target/x86_64-unknown-linux-musl/release/hello_rocket

FROM alpine:latest as final
WORKDIR /app
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/hello_rocket .
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./hello_rocket"]