# Start with the official Rust image as a base
FROM rust:latest

ENV RD=/app

# Install system dependencies needed for building Rust code
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libudev-dev && \
    rm -rf /var/lib/apt/lists/*

# Set a working directory
WORKDIR $RD

COPY ./ .

WORKDIR $RD/wasm

# Execute the build script
RUN ./build

WORKDIR $RD/www

# Execute the build script
RUN ./build

# Command to run on container start (for now, it just keeps the container running)
CMD ["tail", "-f", "/dev/null"]
