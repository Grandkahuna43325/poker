ARG RUST_VERSION=1.74.1
ARG APP_NAME=api
ARG API_PORT=8080

# Create a stage for building the application.
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /usr/src/app

# Install host build dependencies.
RUN apt update
RUN apt install pkg-config libpq-dev -y
# RUN apt install libpq-dev libssl-dev libudev-dev librust-openssl-sys-dev -y
RUN apt-get update -y && \
  apt-get install -y pkg-config libpq-dev make g++ libssl-dev
RUN rustup target add x86_64-unknown-linux-gnu && rustup target add wasm32-unknown-unknown

RUN cargo install trunk wasm-bindgen-cli

COPY . .
ENV DATABASE_URL=postgres://postgres.rwiyuhxmstzcvmpeabna:q00ddM77RdJaF19b@aws-0-eu-central-1.pooler.supabase.com:5432/postgres
RUN cd api && cargo build --release --target-dir ./target 
RUN mv ./api/target/release/$APP_NAME /bin/server
RUN cd /usr/src/app/frontend && trunk build --release



################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the alpine image as the foundation for running the app.
# By specifying the "3.18" tag, it will use version 3.18 of alpine. If
# reproducability is important, consider using a digest
# (e.g., alpine@sha256:664888ac9cfd28068e062c991ebcff4b4c7307dc8dd4df9e728bedde5c449d91).
FROM debian:bullseye-slim AS final
ENV DATABASE_URL=postgres://postgres.rwiyuhxmstzcvmpeabna:q00ddM77RdJaF19b@aws-0-eu-central-1.pooler.supabase.com:5432/postgres
ENV PGPASSWORD=q00ddM77RdJaF19b
ARG API_PORT

RUN apt update
RUN apt install -y libpq-dev libssl-dev

COPY --from=build /bin/server /server
COPY --from=build /usr/src/app/api/css /css
COPY --from=build /usr/src/app/frontend/dist /dist



# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/bin/sh" \
    --no-create-home \
    --uid "${UID}" \
    appuser

RUN chown appuser /server
RUN chown appuser /css
RUN chown appuser /dist

USER appuser

# Copy the executable from the "build" stage.


# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["/server"]
