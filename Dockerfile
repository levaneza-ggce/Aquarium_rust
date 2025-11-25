# Stage 1: Build the Rust application
# We use the official Rust image to compile our code
FROM rust:1.82-slim-bookworm as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build for release to ensure high performance
RUN cargo build --release

# Stage 2: Create a minimal runtime image
# We use a small Debian image because it links easily with Rust binaries
FROM debian:bookworm-slim

# Install any necessary runtime libraries (rarely needed for simple Rust apps, but good practice)
RUN apt-get update && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-aquarium /usr/local/bin/aquarium

# Set the entrypoint to run the application
# Since this is a TUI, it expects an interactive terminal (allocate via -it in docker run)
CMD ["aquarium"]
