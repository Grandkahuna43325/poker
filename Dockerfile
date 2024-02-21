FROM rust:1.74.1 AS build


RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli trunk
WORKDIR /usr/src/app
COPY . .

RUN cd frontend && trunk build --release
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/app/target/release/ /usr/local/bin
COPY --from=build /usr/src/app/frontend/dist/ /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["api"]
