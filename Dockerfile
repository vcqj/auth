# =========================
# ====== BUILDER ==========
# =========================
# zerovec >=0.11 needs rustc >= 1.82 — use a modern stable.
FROM rust:1.83-slim AS builder

# System deps to compile Diesel (postgres feature) and OpenSSL
RUN apt-get update && apt-get install -y --no-install-recommends \
      pkg-config libssl-dev libpq-dev build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first to leverage Docker layer caching
# If you have a Cargo.lock, include it to pin exact versions.
COPY Cargo.toml Cargo.lock ./

# Prebuild deps with a dummy main to cache compilation layers
RUN mkdir -p src && echo "fn main(){}" > src/main.rs
# If resolver hiccups on first cold build, don't fail the layer
RUN cargo build --release || true

# Now copy real sources and build your app
COPY src ./src
RUN cargo build --release

# =========================
# ====== RUNTIME ==========
# =========================
FROM debian:bookworm-slim AS runtime

# libpq is required at runtime for Diesel(Postgres)
RUN apt-get update && apt-get install -y --no-install-recommends \
      libpq5 ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 10001 appuser

WORKDIR /app

# Bring in the compiled binary
COPY --from=builder /app/target/release/auth /usr/local/bin/auth

# Minimal runtime env
ENV RUST_LOG=info \
    BIND=0.0.0.0:8080
EXPOSE 8080

USER appuser
ENTRYPOINT ["auth"]

