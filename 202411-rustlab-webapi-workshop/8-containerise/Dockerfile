FROM rust:1.82-bullseye AS builder

ENV RUSTFLAGS='-C target-feature=+crt-static -C link-arg=-s'

WORKDIR /opt/app

RUN apt-get update && apt-get upgrade -y
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get install -y --no-install-recommends \
  musl-tools

# Create a dummy project to enable dependencies to be cached in layer
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src/

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static-debian12
COPY --from=builder /opt/app/target/x86_64-unknown-linux-musl/release/italia /italia
ENTRYPOINT [ "/italia" ]
