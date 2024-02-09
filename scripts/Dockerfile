FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:$PATH"
RUN apt-get update && \
    apt-get install -y curl git vim build-essential pkg-config libssl-dev && \
    curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup override set nightly

COPY . /home
WORKDIR /home

RUN cargo build