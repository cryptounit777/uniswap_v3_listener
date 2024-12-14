FROM rust:1.82 as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/ethereum-tracker .

CMD ["./ethereum-tracker"]

