# ========== Stage 1: Builder ========== #
FROM rust:1-alpine3.17 as builder

RUN apk add musl-dev

WORKDIR /usr/src/grandmondebot

COPY . .

RUN cargo install --path .

# ========= Stage 2: Production ========= #
FROM alpine as production

RUN apk add docker

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/grandmondebot /app/grandmondebot

CMD ./grandmondebot