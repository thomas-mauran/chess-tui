# 1. This tells docker to use the Rust official image
FROM rust:1.83 as builder

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

FROM debian:bookworm-slim AS runner
COPY --from=builder /target/release/chess-tui /usr/bin/chess-tui

ENTRYPOINT [ "/usr/bin/chess-tui" ]