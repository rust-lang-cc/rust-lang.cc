# based on https://github.com/fly-apps/hello-rust/blob/main/Dockerfile
# 要清楚哪里需要编译的可执行文件，目前是 web 目录下需要，最终的 workdir 下面最需要有一个
# removed builder image steps

# Runtime image
# FROM debian:bullseye-slim  94MB with apt-get update && apt-get install -y wget && rm -rf /var/lib/apt/lists/*
FROM debian:bullseye-slim

# Get compiled binaries from builder's cargo install directory
# COPY --from=builder /root/.cargo/bin/rustlangcc /app/

# Copy the rest
COPY . /app/

# Run as "app" user
# RUN useradd -ms /bin/bash app
RUN apt-get update && apt-get install -y wget && rm -rf /var/lib/apt/lists/* && \
useradd -ms /bin/bash app && \
wget https://rust-lang.cc/rustlangcc && chmod +x rustlangcc && mkdir -p /app/front-end/ && cp rustlangcc /app/front-end/ &&  cp rustlangcc /app/ && \
wget https://github.com/rust-lang/mdBook/releases/download/v0.4.12/mdbook-v0.4.12-x86_64-unknown-linux-gnu.tar.gz && \
tar -zxf mdbook-v0.4.12-x86_64-unknown-linux-gnu.tar.gz && rm -f mdbook-v0.4.12-x86_64-unknown-linux-gnu.tar.gz && \
chmod +x mdbook &&  cp mdbook /app/ && chown -R app:app /app && chmod 755 /app

# USER app
USER app
WORKDIR /app

RUN /app/mdbook build /app/cn-mdbook && /app/mdbook build /app/en-mdbook

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.
