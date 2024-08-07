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

RUN --mount=type=bind,source=.,target=/app,rw \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/var/cache/apt/ \
    cargo build --locked --release

RUN --mount=type=cache,target=/app/target/ \
    cp /app/target/release/$APP_NAME /bin


# ---

FROM docker.io/library/debian:bullseye-slim AS final
ARG APP_NAME

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

USER 10001

COPY --from=build /bin/$APP_NAME /bin/

CMD ["/bin/prometheus-feed-exporter"]
