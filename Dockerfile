FROM rust:1.60-slim as builder

RUN cargo new --bin app
WORKDIR /app

COPY --link . .

RUN --mount=type=cache,target="/app/target/release" \
    cargo build --release && mv target/release/vsts .

#--------------------------------------------------
FROM ubuntu:22.04

WORKDIR /app

RUN apt-get update && apt-get install tini

COPY --from=builder /app/vsts .

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/app/vsts"]