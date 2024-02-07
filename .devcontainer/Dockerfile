FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# [Optional] Uncomment this section to install additional packages.
RUN export DEBIAN_FRONTEND=noninteractive && apt-get update && apt-get upgrade \
    && apt-get -y install --no-install-recommends \
    protobuf-compiler \
    libprotobuf-dev \
    g++ \
    pkg-config \
    libx11-dev \
    libasound2-dev \
    libudev-dev \
    libxkbcommon-x11-0 \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    mesa-vulkan-drivers
