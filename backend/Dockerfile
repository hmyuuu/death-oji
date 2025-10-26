FROM rust:1.83 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY partition.json ./

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/death-oji-backend .
COPY --from=builder /app/partition.json .

EXPOSE 11451

CMD ["./death-oji-backend"]
