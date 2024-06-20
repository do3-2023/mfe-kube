FROM clux/muslrust:stable as builder

RUN apt update && \
    apt upgrade

FROM builder as build

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

COPY --from=build --chown=1000 /app/target/x86_64-unknown-linux-musl/release/mfe-api /

ENV APP_PORT=80
EXPOSE ${APP_PORT}

CMD ["/mfe-api"]