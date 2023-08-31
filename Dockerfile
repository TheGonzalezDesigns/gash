# Start with the official Rust image as a base
FROM rust:latest

ENV RD=/app

# Install system dependencies for three-rs and X11
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    libx11-xcb-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxcb-randr0-dev \
    libxcb-image0-dev \
    libgl1-mesa-dev \
    libgl1-mesa-glx \
    libglu1-mesa-dev \
    freeglut3-dev \
    mesa-common-dev \
    libx11-xcb1 \
    libxcb1 \
    libxkbcommon-x11-0 \
    libxkbcommon0 \
    libdbus-1-3 \
    libwayland-client0 \
    libwayland-cursor0 \
    libwayland-egl1 && \
    rm -rf /var/lib/apt/lists/*

# Set a working directory
WORKDIR $RD

WORKDIR $RD/wasm

COPY wasm/ .

RUN chown -R root:root .

# Execute the build script
RUN ./build

# Command to run on container start (for now, it just keeps the container running)
CMD ["tail", "-f", "/dev/null"]
