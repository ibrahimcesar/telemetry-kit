# Build stage
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY telemetry-kit-server/Cargo.toml ./telemetry-kit-server/

# Create dummy source files to cache dependencies
RUN mkdir -p telemetry-kit-server/src && \
    echo "fn main() {}" > telemetry-kit-server/src/main.rs

# Build dependencies
RUN cargo build --release --package telemetry-kit-server

# Copy actual source code
COPY telemetry-kit-server/src ./telemetry-kit-server/src
COPY telemetry-kit-server/migrations ./telemetry-kit-server/migrations

# Build the application
RUN cargo build --release --package telemetry-kit-server

# Runtime stage
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache ca-certificates libgcc

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/telemetry-kit-server ./telemetry-kit-server

# Expose port
EXPOSE 3000

# Run the server
CMD ["./telemetry-kit-server"]
