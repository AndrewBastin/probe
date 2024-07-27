FROM alpine:3.20

RUN apk add build-base rust cargo nodejs npm g++ zlib zlib-dev supervisor caddy icu-libs icu-data-full cargo-watch openssl-dev
RUN npm install -g pnpm

CMD ["supervisord", "-c", "/fosshack/dev-supervisord.conf"]
