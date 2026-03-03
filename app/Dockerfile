FROM rust:1.91-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    perl \
    perl-modules \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

RUN ARCH=$(uname -m) && \
    case "$ARCH" in \
      x86_64)  TARGET="x86_64-unknown-linux-gnu" ;; \
      aarch64) TARGET="aarch64-unknown-linux-gnu" ;; \
      *) echo "Arquitectura no soportada: $ARCH" && exit 1 ;; \
    esac && \
    curl -L --proto '=https' --tlsv1.2 \
      "https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.5/cargo-leptos-${TARGET}.tar.gz" \
      -o /tmp/cargo-leptos.tar.gz && \
    tar -xzf /tmp/cargo-leptos.tar.gz -C /tmp && \
    find /tmp -name "cargo-leptos" -type f | head -1 | xargs -I{} mv {} /usr/local/bin/cargo-leptos && \
    chmod +x /usr/local/bin/cargo-leptos && \
    rm /tmp/cargo-leptos.tar.gz && \
    cargo-leptos --version

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY style ./style
COPY public ./public
COPY . .

ARG API_BASE_URL=http://server:4000
ARG PUBLIC_API_BASE_URL=http://localhost:55547

ENV API_BASE_URL=${API_BASE_URL}
ENV PUBLIC_API_BASE_URL=${PUBLIC_API_BASE_URL}
ENV LEPTOS_OUTPUT_NAME=stego-app
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_PKG_DIR=pkg

RUN cargo leptos build --release

FROM debian:trixie-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/stego-app .
COPY --from=builder /app/site ./site
COPY --from=builder /app/public ./public

ENV LEPTOS_OUTPUT_NAME=stego-app
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=0.0.0.0:3001
EXPOSE 3001
CMD ["./stego-app"]