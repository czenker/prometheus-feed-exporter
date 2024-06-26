# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.75.0
ARG APP_NAME=prometheus-feed-exporter

# ---

FROM docker.io/library/rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN --mount=type=bind,source=.,target=/app,rw,relabel=shared \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/var/cache/apt/ \
    cargo build --locked --release

# ---

FROM gcr.io/distroless/static-debian12 AS final
ARG APP_NAME

USER 10001

COPY ./target/release/$APP_NAME /bin/

CMD ["/bin/$APP_NAME"]