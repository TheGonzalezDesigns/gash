# Start with the official Rust image as a base
FROM rust:latest

# Install system dependencies for three-rs
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
    mesa-common-dev && \
    rm -rf /var/lib/apt/lists/*

# Set a working directory
WORKDIR /app

# Command to run on container start (for now, it just keeps the container running)
CMD ["tail", "-f", "/dev/null"]
