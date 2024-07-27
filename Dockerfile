FROM alpine:3.20 as backend_builder

RUN apk add build-base rust cargo g++ zlib zlib-dev openssl-dev

WORKDIR /fosshack
COPY backend .

RUN cargo build --release



FROM alpine:3.20 as backend

RUN apk add openssl-dev

ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT=8000

WORKDIR /fosshack
COPY --from=backend_builder /fosshack/target/release/fosshack .

EXPOSE 8000
ENTRYPOINT "/fosshack/fosshack"



FROM node:20-alpine3.20 as frontend_builder

RUN apk add icu-libs icu-data-full
RUN npm install -g pnpm

WORKDIR /frontend
COPY frontend .

RUN pnpm install
RUN NITRO_PRESET=node-server pnpm run build



FROM node:20-alpine3.20 as frontend

WORKDIR /frontend
COPY --from=frontend_builder /frontend/.output .

EXPOSE 3000

ENTRYPOINT ["node", "/frontend/server/index.mjs"]



FROM node:20-alpine3.20 as aio

RUN apk add supervisor caddy icu-libs icu-data-full

WORKDIR /fosshack
COPY --from=backend_builder /fosshack/target/release/fosshack_backend backend
COPY --from=frontend_builder /frontend/.output frontend
COPY aio-supervisord.conf supervisord.conf
COPY Caddyfile Caddyfile

ENV ROCKET_PORT=8000
ENV ROCKET_ADDRESS="0.0.0.0"

CMD ["supervisord", "-c", "/fosshack/supervisord.conf"]
