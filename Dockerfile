# 1. This tells docker to use the Rust official image
FROM rust:1.87 as builder

# Install ALSA development libraries for rodio
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libasound2-dev \
    && rm -rf /var/lib/apt/lists/*

# 2. Copy dependency files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# 3. Create a dummy src directory to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 4. Copy the actual source code
COPY src/ ./src/

# 5. Build your program for release
RUN cargo build --release

FROM debian:bookworm-slim AS runner

# Install SSL libraries required by reqwest and ALSA runtime libraries for rodio
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl3 \
    libasound2 \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Configure ALSA to use a null device to suppress error messages when no audio hardware is available
# This prevents ALSA from spamming stderr with errors in Docker containers
RUN echo 'pcm.!default { type null }' > /etc/asound.conf && \
    echo 'ctl.!default { type null }' >> /etc/asound.conf

COPY --from=builder /target/release/chess-tui /usr/bin/chess-tui

ENTRYPOINT [ "/usr/bin/chess-tui" ]