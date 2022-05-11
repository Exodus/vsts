FROM rust:1.60-slim as builder

RUN cargo new --bin app
WORKDIR /app

COPY Cargo.toml Cargo.lock .

RUN cargo build --release && \
    rm src/*.rs

COPY src src
RUN --mount=type=cache,target=target/release rm target/release/deps/vsts* && \
    cargo build --release

#--------------------------------------------------
FROM ubuntu:22.04

WORKDIR app/

RUN apt-get update && apt-get install tini

COPY --from=builder /app/target/release/vsts .

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/app/vsts"]