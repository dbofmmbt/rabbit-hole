ARG appName=producer

FROM rust:latest as builder
ARG appName

# setup project
RUN cargo new --bin $appName
WORKDIR /$appName

# build dependencies only
COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo build --release && rm -r src target/release/deps/${appName}*

# build the project
COPY . .
RUN cargo build --release

# ---

FROM debian:buster-slim
ARG appName

COPY --from=builder /$appName/target/release/$appName ./app

CMD ["./app"]