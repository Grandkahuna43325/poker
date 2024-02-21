FROM rust:1.74.1 AS build


RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli trunk
WORKDIR /usr/src/app
COPY . .

RUN cd frontend && trunk build --release
RUN cargo build --release

FROM debian as final

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev
RUN apt-get install -y postgresql-client


COPY --from=build /usr/src/app/target/release/ /usr/local/bin
COPY --from=build /usr/src/app/api/css/ /usr/local/css
COPY --from=build /usr/src/app/frontend/dist/ /usr/local/dist

WORKDIR /usr/local
CMD ["/usr/local/bin/api"]
