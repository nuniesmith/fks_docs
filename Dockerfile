ARG RUST_VERSION=1.82
ARG TARGETPLATFORM
ARG BUILDPLATFORM
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-slim AS builder

WORKDIR /app
ENV CARGO_TERM_COLOR=always

# System deps (kept minimal)
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY Cargo.lock* ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && cargo fetch

# Real sources
RUN rm -rf src
COPY src/ src/
COPY docs/ docs/

RUN echo "Building for TARGETPLATFORM=$TARGETPLATFORM" && cargo build --release --locked || cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/* && update-ca-certificates
COPY --from=builder /app/target/release/fks_docs /usr/local/bin/fks_docs
COPY docs/ docs/
RUN useradd -r -s /bin/false -u 1000 fks_docs && chown -R fks_docs:fks_docs /app
USER fks_docs
EXPOSE 8040
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 CMD curl -fsS http://localhost:8040/health || exit 1
ENV FKS_DOCS_PORT=8040 FKS_DOCS_DIR=docs FKS_DOCS_RENDER_MD_HTML=1
CMD ["/usr/local/bin/fks_docs"]

