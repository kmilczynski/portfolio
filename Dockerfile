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
FROM rust:1.83-slim AS builder

WORKDIR /app

# Install required build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack and perseus CLI
RUN cargo install wasm-pack
RUN cargo install perseus-cli --version 0.4.2

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
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

# Copy content files
COPY --from=builder /app/src/content/ ./src/content/

# Expose port (adjust if needed)
EXPOSE 8080

# Set environment variables
ENV PERSEUS_STANDALONE=true
ENV RUST_LOG=info

# Run the server
CMD ["./pkg/server"]
