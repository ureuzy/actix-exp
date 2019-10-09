FROM ekidd/rust-musl-builder:stable

WORKDIR /usr/src/app

ADD . .

RUN sudo apt-get update && sudo apt-get install -y \
    libmysqlclient-dev \
    openssl
RUN sudo chown -R rust:rust /usr/src
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --update --no-cache mariadb-client

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/actix-exp /bin/

CMD actix-exp
