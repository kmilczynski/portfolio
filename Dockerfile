# Stage 1: Build CSS with Node.js
FROM node:20-alpine AS css-builder

WORKDIR /app

# Copy package files
COPY package.json package-lock.json* ./

# Install dependencies
RUN npm install

# Copy CSS source files
COPY style/ ./style/
COPY static/ ./static/

# Build minified CSS
RUN npm run prod

# Stage 2: Build Perseus application
FROM rust:1.91-slim AS builder

WORKDIR /app

# Install required build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    binaryen \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack and perseus CLI
RUN rustup default 1.91.0 && rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack
RUN cargo install wasm-bindgen-cli --version 0.2.106
RUN cargo install perseus-cli --locked

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY migrations/ ./migrations
COPY translations ./translations
COPY .cargo/ ./.cargo/

# Copy source code
COPY src/ ./src/
COPY static/ ./static/

# Copy minified CSS from css-builder stage
COPY --from=css-builder /app/static/css/main.css ./static/css/main.css

# Build Perseus application in production mode
RUN perseus deploy

# Stage 3: Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built application from builder
COPY --from=builder /app/pkg/ ./pkg/

# Copy static assets
COPY --from=builder /app/static/ ./static/

# Copy translations
COPY --from=builder /app/translations/ ./translations/

# Copy content files
COPY --from=builder /app/src/content/ ./src/content/

# Copy migrations and create data directory for SQLite
COPY --from=builder /app/migrations/ ./migrations/
RUN mkdir -p ./data

# Expose port (adjust if needed)
EXPOSE 8080

# Set environment variables
ENV PERSEUS_STANDALONE=true
ENV PERSEUS_HOST=::
ENV RUST_LOG=info

# Run the server
CMD ["./pkg/server"]
