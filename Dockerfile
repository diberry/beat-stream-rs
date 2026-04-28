# Stage 1: Build
FROM rust:1.82-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* rust-toolchain.toml ./
COPY crates/ crates/

# Build release binary
RUN cargo build --release --bin beat-stream-server

# Stage 2: Runtime
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/beat-stream-server /beat-stream-server

EXPOSE 8080

ENTRYPOINT ["/beat-stream-server"]
