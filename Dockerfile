FROM rust:1.99.1-trixie as builder
WORKDIR /usr/src/p2p-probe
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM debian:trixie
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/p2p-probe /usr/local/bin/p2p-probe
ENTRYPOINT ["p2p-probe"]
