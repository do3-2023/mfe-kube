FROM clux/muslrust:stable as builder

RUN apt update && \
    apt upgrade && \
    curl -L https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.4/tailwindcss-linux-x64 -o /usr/local/bin/tailwindcss && \
    chmod +x /usr/local/bin/tailwindcss

FROM builder as build

WORKDIR /app

RUN cargo init
COPY ./Cargo.* .
RUN cargo fetch

COPY ./assets ./assets
COPY ./templates ./templates
COPY ./src ./src
COPY ./tailwind.config.js ./tailwind.config.js

RUN cargo build --target x86_64-unknown-linux-musl --release

RUN /usr/local/bin/tailwindcss -c ./tailwind.config.js -i ./assets/main.css -o ./dist/main.css --minify

FROM alpine:3 as runtime

RUN apk update && \
    apk upgrade && \
    apk add openssl-dev

RUN addgroup -S mfe-web-user && \
    adduser -S mfe-web-user -G mfe-web-user

WORKDIR /app

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/mfe-web /app/mfe-web
COPY --from=build /app/dist /app/dist

USER mfe-web-user

ENV APP_PORT=80
EXPOSE ${APP_PORT}

CMD ["/app/mfe-web"]