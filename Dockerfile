# ---------- build stage ----------
FROM rust:1.90-slim AS builder

WORKDIR /app

# system deps
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# install perseus
RUN cargo install perseus-cli --locked

# copy source
COPY . .

# build perseus app
RUN perseus build --release

# ---------- runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app /app
COPY --from=builder /usr/local/cargo/bin/perseus /usr/local/bin/perseus

EXPOSE 556

CMD ["perseus", "serve", "--release", "--host", "::", "--port", "556"]
