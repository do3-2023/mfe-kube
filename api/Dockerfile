FROM rust:1-slim-bookworm as builder

RUN apt update && apt install -y musl-tools musl-dev
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
ENV SQLX_OFFLINE=true

RUN cargo init
COPY ./Cargo.* .
RUN cargo fetch

COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx
COPY ./src ./src

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

USER 1000

COPY --from=builder --chown=1000 /app/target/x86_64-unknown-linux-musl/release/mfe-api /

ENV APP_PORT=80
EXPOSE 80

CMD ["/mfe-api"]