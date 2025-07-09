ARG CARGO_HOME=/opt/cargo
FROM --platform=linux/amd64 debian:bookworm-slim AS base

WORKDIR /base

RUN dpkg --add-architecture arm64
RUN dpkg --add-architecture armhf

RUN apt-get update && \
    apt-get --yes --no-install-recommends install \
    ca-certificates curl g++ g++-arm-linux-gnueabihf g++-aarch64-linux-gnu && \
    apt-get --yes clean && \
    rm -rf /var/lib/apt/lists/*

# Rust toolchain
## Use the toolchain settings from the project
COPY rust-toolchain.toml .
ARG CARGO_HOME
ENV RUSTUP_HOME=/opt/rustup
RUN (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none)
ENV PATH="${PATH}:${CARGO_HOME}/bin"

## This will install the toolchain and components from rust-toolchain.toml
RUN rustup toolchain install

## Targets have to be added additionally
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup target add armv7-unknown-linux-gnueabihf

FROM base AS build

ARG TARGET=x86_64-unknown-linux-gnu
ARG BUILD_TYPE="debug"
ENV CARGO_ARGS="--target ${TARGET} --path template_bin"

WORKDIR /src
COPY . .

RUN if [ "$BUILD_TYPE" = "debug" ]; then \
      cargo install $CARGO_ARGS --debug; \
    else \
      cargo install $CARGO_ARGS; \
    fi

FROM debian:bookworm-slim
ARG CARGO_HOME

COPY --from=build $CARGO_HOME/bin/template_bin /

ENTRYPOINT [ "/template_bin" ]