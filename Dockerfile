FROM rust:slim-bookworm AS build
WORKDIR /build
COPY . .
RUN cargo build --release
FROM debian:bookworm-slim
COPY --from=build /build/target/release/endlessmeow /endlessmeow
ENTRYPOINT ["/endlessmeow"]
