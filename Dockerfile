FROM rust:1-alpine AS builder

LABEL org.opencontainers.image.source=https://github.com/zoobestik/borken_botdo

RUN apk update && apk upgrade && rm -rf /var/cache/apk/*

RUN apk add --no-cache \
    build-base \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    zlib-static \
    sqlite-dev

# Hint crates to link statically to OpenSSL when available
ENV OPENSSL_STATIC=1 \
    PKG_CONFIG_ALL_STATIC=1

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM alpine

COPY --from=builder /usr/src/app/target/release/bot_cli /usr/local/bin/bot

WORKDIR "/app"

CMD ["bot"]