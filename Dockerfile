FROM rust:slim

WORKDIR /usr/src/hybrid-cli

COPY . .

RUN cargo build --release -j 1
