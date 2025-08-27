# ------- builder stage -------
FROM rust:1.79-slim AS builder

# System deps to compile Diesel (postgres feature) and OpenSSL
RUN apt-get update && apt-get install -y --no-install-recommends \
      pkg-config libssl-dev libpq-dev build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache deps first
COPY Cargo.toml Cargo.lock ./
# Create a dummy main to warm the cache
RUN mkdir -p src && echo "fn main(){}" > src/main.rs
RUN cargo build --release || true

# Copy real sources and build
COPY src ./src
RUN cargo build --release

# ------- runtime stage -------
FROM debian:bookworm-slim AS runtime

# libpq is required at runtime for Diesel(Postgres)
RUN apt-get update && apt-get install -y --no-install-recommends \
      libpq5 ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 10001 appuser

WORKDIR /app
COPY --from=builder /app/target/release/auth /usr/local/bin/auth

ENV RUST_LOG=info \
    BIND=0.0.0.0:8080
EXPOSE 8080

USER appuser
ENTRYPOINT ["auth"]

