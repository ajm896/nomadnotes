FROM rust:slim-bullseye AS builder

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src

RUN cargo build --release


FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/nomadnotes .

EXPOSE 4000
CMD ["./nomadnotes"]