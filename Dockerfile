# ╔══════════════════════════════════════════════════╗
# ║  MCP Sidecar — Multi-stage Rust Build            ║
# ╚══════════════════════════════════════════════════╝

FROM rust:1.85-slim AS builder

WORKDIR /app

# Install OpenSSL for reqwest
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -f target/release/deps/hermodr*

# Build real binary
COPY src/ src/
RUN cargo build --release

# Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/hermodr /usr/local/bin/hermodr

# Configured via env vars:
#   SERVICE_NAME  — "yggdrasil" or "eir" (selects tool set)
#   UPSTREAM_URL  — base URL to proxy REST calls to
#   PORT          — listen port (default: 8090)

EXPOSE 8090
CMD ["hermodr"]
