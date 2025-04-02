FROM mcr.microsoft.com/devcontainers/base:ubuntu

# Arguments
ARG USER=vscode

# Install dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    build-essential \
    curl \
    wget \
    git \
    pkg-config \
    libssl-dev \
    libffi-dev \
    software-properties-common \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
USER ${USER}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/${USER}/.cargo/bin:${PATH}"

# Install Python 3.10
USER root
RUN add-apt-repository ppa:deadsnakes/ppa \
    && apt-get update \
    && apt-get install -y python3.10 python3.10-venv python3.10-dev python3-pip \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* \
    && ln -s /usr/bin/python3.10 /usr/local/bin/python \
    && ln -s /usr/bin/python3.10 /usr/local/bin/python3 \
    && curl -sS https://bootstrap.pypa.io/get-pip.py | python3.10

# Install Python data science packages
RUN pip install --no-cache-dir \
    polars \
    pandas \
    matplotlib \
    jupyter \
    ipykernel \
    numpy \
    scikit-learn \
    uv

# Set user for subsequent commands
USER ${USER}

# Install Rust components
RUN rustup component add clippy rustfmt \
    && cargo install cargo-watch cargo-edit cargo-update

# Set default shell to bash
ENV SHELL=/bin/bash