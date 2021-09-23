# based on https://github.com/fly-apps/hello-rust/blob/main/Dockerfile
# 要清楚哪里需要编译的可执行文件，目前是 web 目录下需要，最终的 workdir 下面最需要有一个
FROM liuchong/rustup:stable AS builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs
# Will build all dependent crates in release mode
RUN cargo build --release

# Copy the rest
COPY . .
# Build (install) the actual binaries
RUN cargo install --path .

# Runtime image
# FROM debian:bullseye-slim  94MB with apt-get update && apt-get install -y wget && rm -rf /var/lib/apt/lists/*
FROM debian:bullseye-slim

# Copy the rest
COPY . /app/

# Get compiled binaries from builder's cargo install directory
# COPY --from=builder /root/.cargo/bin/rustlangcc /app/
COPY --from=builder /root/.cargo/bin/rustlangcc /

# Run as "app" user
# RUN useradd -ms /bin/bash app
# install wget && wget mdbook && mdbook build
RUN apt-get update && apt-get install -y wget && rm -rf /var/lib/apt/lists/* && \
useradd -ms /bin/bash app && \
chmod +x rustlangcc && \
mkdir -p /app/front-end/ && cp rustlangcc /app/front-end/ && cp rustlangcc /app/ && \
wget https://github.com/rust-lang/mdBook/releases/download/v0.4.12/mdbook-v0.4.12-x86_64-unknown-linux-gnu.tar.gz && \
tar -zxf mdbook-v0.4.12-x86_64-unknown-linux-gnu.tar.gz && rm -f mdbook-v0.4.12-x86_64-unknown-linux-gnu.tar.gz && \
chmod +x mdbook &&  cp mdbook /app/ && chown -R app:app /app && chmod 755 /app

USER app
WORKDIR /app

RUN /app/mdbook build /app/cn-mdbook

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.
