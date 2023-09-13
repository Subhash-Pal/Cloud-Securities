# Use the official Rust image as the base image
FROM rust:1.71.1 AS rust_builder 

# Install necessary tools
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rm -rf /var/lib/apt/lists/*

# Configure aya toolchain
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup toolchain install nightly --component rust-src
RUN cargo install bpf-linker

# Copy the Rust project files
COPY . /usr/src/app

# Set the working directory for the Rust project
WORKDIR /usr/src/app

# Build the Rust project
RUN cd xdp-drop \
    && cargo install cargo-xtask && \
    cargo xtask build-ebpf --release && \
    cargo build --target x86_64-unknown-linux-musl --release

# Final image with only the necessary artifacts
FROM ubuntu:20.04

# Set the working directory inside the container
WORKDIR /app

# Copy the built Rust binary from the rust_builder stage
COPY --from=rust_builder /usr/src/app/xdp-drop/target/release/xdp-drop ./xdp-drop

# Set the entry point for the container to run the xdp-drop binary
ENTRYPOINT ["./xdp-drop"]
