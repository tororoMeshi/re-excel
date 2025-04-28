#!/bin/bash
set -eux

IMAGE_NAME=rust-lint-extended

# Dockerイメージをビルド（キャッシュ可）
docker build -t "$IMAGE_NAME" - <<EOF
FROM rust:1.86-slim

RUN rustup component add rustfmt clippy && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev libwebp-dev \
        git curl && \
    cargo install cargo-deny cargo-outdated && \
    rm -rf /var/lib/apt/lists/*
EOF

# 現在のRustプロジェクトをマウントしてチェック実行
docker run --rm \
  -v "$PWD":/usr/src/app \
  -w /usr/src/app \
  "$IMAGE_NAME" \
  bash -c "
    cargo fmt --all &&
    cargo check &&
    cargo clippy -- -D warnings &&
    cargo deny check &&
    cargo outdated || true
  "
