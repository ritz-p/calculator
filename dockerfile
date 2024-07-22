FROM rust:latest

RUN apt update

RUN rustup component add rls rust-analysis rust-src rustfmt clippy

WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1