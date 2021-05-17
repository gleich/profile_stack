# hadolint ignore=DL3007
FROM rust:latest AS builder

# Meta data
LABEL maintainer="email@mattglei.ch"
LABEL description="🚀 Display your tech stack on your GitHub profile's README"

# File copy
COPY . /usr/src/app
WORKDIR /usr/src/app

# Setup nightly
RUN rustup toolchain install nightly && \
    rustup default nightly

# Install cargo-make
ENV CARGO_MAKE_VERSION 0.33.0
ENV CARGO_MAKE_TMP_DIR /tmp/setup-rust-cargo-make
RUN mkdir ${CARGO_MAKE_TMP_DIR} && \
    wget -qO ${CARGO_MAKE_TMP_DIR}/cargo-make.zip https://github.com/sagiegurari/cargo-make/releases/download/${CARGO_MAKE_VERSION}/cargo-make-v${CARGO_MAKE_VERSION}-x86_64-unknown-linux-musl.zip && \
    unzip -d ${CARGO_MAKE_TMP_DIR} ${CARGO_MAKE_TMP_DIR}/cargo-make.zip && \
    mv ${CARGO_MAKE_TMP_DIR}/cargo-make-v${CARGO_MAKE_VERSION}-x86_64-unknown-linux-musl/cargo-make /usr/local/bin

# Binary build
RUN cargo make build-rust-dev

# Copy of binary to smaller image
# hadolint ignore=DL3006,DL3007
FROM debian:stable-slim
WORKDIR /
COPY --from=builder /usr/src/app/target/debug/profile_stack .

# Install needed deps
# hadolint ignore=DL3008
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends libpq5 ca-certificates libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Setting env vars
ENV RUST_LOG info
ENV RUST_BACKTRACE 1

CMD ["./profile_stack"]
