FROM rust:1.77-bookworm as builder
WORKDIR /usr/src/p2p-probe
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM debian:bookworm
COPY --from=builder /usr/local/cargo/bin/p2p-probe /usr/local/bin/p2p-probe
ENTRYPOINT ["p2p-probe"]
