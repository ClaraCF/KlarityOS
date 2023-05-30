FROM rust

# Configure Rust environment
RUN rustup target add x86_64-unknown-none
RUN rustup install nightly
RUN rustup default nightly
RUN rustup component add rust-src
RUN rustup component add llvm-tools-preview

# Install additional components
RUN cargo install bootimage

VOLUME /root/env
WORKDIR /root/env