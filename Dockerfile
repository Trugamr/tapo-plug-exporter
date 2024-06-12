FROM rust:1.78-bookworm as base

####################

FROM base as builder

WORKDIR /app

COPY . .  

RUN cargo build --release

####################

FROM debian:bookworm-slim as runner

WORKDIR /usr/local/bin

RUN apt update && apt install -y openssl

COPY --from=builder /app/target/release/tapo-plug-exporter .

ENTRYPOINT ["tapo-plug-exporter"]
