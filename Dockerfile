FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

RUN useradd -m appuser
USER appuser

WORKDIR /home/appuser

COPY --from=builder /usr/src/app/target/release/memelibre_client .
COPY --from=builder /usr/src/app/src/templates ./src/templates
COPY --from=builder /usr/src/app/src/public ./src/public

EXPOSE 3001

CMD ["./memelibre_client"]
