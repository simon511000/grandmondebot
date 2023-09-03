# ========== Stage 1: Builder ========== #
FROM rust:1-alpine3.17 as builder

RUN apk add musl-dev

RUN cargo new /app/grandmondebot
COPY ./Cargo.toml ./Cargo.lock /app/grandmondebot/

WORKDIR /app/grandmondebot

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

# ========= Stage 2: Production ========= #
FROM alpine as production

RUN apk add docker

WORKDIR /app

COPY --from=builder /app/grandmondebot/target/release/grandmondebot /app/grandmondebot

CMD ./grandmondebot