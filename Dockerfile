FROM ekidd/rust-musl-builder:stable as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --update --no-cache mariadb-client

COPY --from=builder /usr/src/app/target/release/actix-exp /bin/

CMD actix-exp