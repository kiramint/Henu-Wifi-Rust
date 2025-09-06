FROM alpine:latest
LABEL authors="kira"

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk update

RUN apk add firefox
RUN apk add --no-cache tzdata

RUN mkdir -p /app

COPY ./target/aarch64-unknown-linux-musl/release/Henu-Wifi-Rust /app/Henu-Wifi-Rust

COPY ./gecko/arm/geckodriver /app/geckodriver

RUN chmod +x /app/Henu-Wifi-Rust
RUN chmod +x /app/geckodriver

ENTRYPOINT ["/app/Henu-Wifi-Rust"]