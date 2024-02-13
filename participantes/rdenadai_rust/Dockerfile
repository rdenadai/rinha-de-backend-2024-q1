FROM rust:1.76.0-alpine3.19

RUN apk add --no-cache musl-dev

WORKDIR /app

ADD ./Cargo.toml .
ADD ./src ./src

RUN cargo install --path .

ENTRYPOINT ["rdenadai_rust"]