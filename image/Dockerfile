FROM debian:stable

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    ca-certificates \
    curl \
    git \
    pkg-config \
    libssl-dev \
    zlib1g-dev

ARG RUST_VERSION=stable

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain ${RUST_VERSION} -y

ENV PATH=/root/.cargo/bin:$PATH

RUN cargo install just

VOLUME /work

WORKDIR /work
