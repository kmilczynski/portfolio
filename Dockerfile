FROM rust:1.90-slim AS builder

WORKDIR /app

# system deps (binaryen provides wasm-opt)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    binaryen \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

RUN cargo install perseus-cli --locked
RUN cargo install wasm-bindgen-cli --locked

COPY . .

RUN perseus deploy

# ---------- runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/pkg/ /app/portfolio
RUN chmod +x /app/portfolio/server

ENV PERSEUS_HOST=0.0.0.0
ENV PERSEUS_PORT=8080

CMD ["./portfolio/server"]