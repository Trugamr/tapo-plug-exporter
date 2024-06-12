VERSION 0.8
FROM clux/muslrust:1.78.0-stable
WORKDIR /app

build:
  COPY . .
  RUN cargo build --release
  
  SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/tapo-plug-exporter

docker:
  FROM gcr.io/distroless/cc-debian12

  COPY +build/tapo-plug-exporter .

  EXPOSE 3456
  ENTRYPOINT ["./tapo-plug-exporter"]

  SAVE IMAGE tapo-plug-exporter:latest