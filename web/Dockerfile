FROM node:18-alpine3.18 as build

WORKDIR /app

COPY . /app

RUN yarn install

RUN yarn build

FROM node:18-alpine3.17

WORKDIR /app

RUN apk update
RUN apk upgrade
RUN apk add dumb-init

USER 1000

COPY --from=build --chown=1000 /app/.output ./

ENV PORT=80
ENV HOST=0.0.0.0
ENV NODE_ENV=production

EXPOSE ${PORT}

# start the app with dumb init to spawn the Node.js runtime process
# with signal support
CMD ["dumb-init", "node", "/app/server/index.mjs"]