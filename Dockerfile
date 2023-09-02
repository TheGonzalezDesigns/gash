# Start with the official Rust image as a base
FROM rust:latest

ENV RD=/app

# Install system dependencies needed for building Rust code
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libudev-dev \
    rsync && \
    rm -rf /var/lib/apt/lists/*

# Set a working directory
WORKDIR $RD

COPY ./ .

WORKDIR $RD/wasm

# Execute the build script
RUN ./build

RUN ./test

WORKDIR $RD/www

# Execute the build script
RUN ./build

# Start the server
CMD ["/root/.bun/bin/bun", "x", "serve", "-n", "-p", "9876", "./production/"]

#CMD ["tail", "-f", "/dev/null"]
