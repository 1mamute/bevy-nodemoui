FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN distribution=$(. /etc/os-release;echo $ID$VERSION_ID | sed -e 's/\.//g') && \
    wget https://developer.download.nvidia.com/compute/cuda/repos/$distribution/x86_64/cuda-keyring_1.0-1_all.deb && \
    dpkg -i cuda-keyring_1.0-1_all.deb && \
    export DEBIAN_FRONTEND=noninteractive && \
    apt-get -y update && \
    apt-get -y install cuda-toolkit && \
    rm cuda*

# [Optional] Uncomment this section to install additional packages.
RUN export DEBIAN_FRONTEND=noninteractive && apt-get -y upgrade \
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
    mesa-vulkan-drivers \
    lld \
    clang

# Cargo tools for nightly must be installed with vscode user
USER vscode
RUN rustup toolchain install nightly && \
    rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu && \
    rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu