ARG RUST_VERSION=1.77.2
ARG APP_NAME=huddle
ARG RUST_BUILD_PROFILE=release
ARG RUST_TARGET_SUBDIR=release
FROM rust:${RUST_VERSION}-slim-bullseye AS build-stage
ARG APP_NAME
ARG RUST_BUILD_PROFILE
ARG RUST_TARGET_SUBDIR
WORKDIR /app

COPY Cargo.* ./
COPY src ./src

RUN echo cargo build --locked --target-dir ./target/$RUST_BUILD_PROFILE --profile $RUST_BUILD_PROFILE
RUN echo cp ./target/$RUST_BUILD_PROFILE/$APP_NAME /bin/server

RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --profile $RUST_BUILD_PROFILE
cp ./target/$RUST_TARGET_SUBDIR/$APP_NAME /bin/server
EOF

FROM debian:stable-slim as production-stage

COPY --from=build-stage /bin/server /bin/

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

CMD ["/bin/server"]