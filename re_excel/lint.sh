#!/usr/bin/env bash
set -euo pipefail

LINT_IMAGE=rust-lint-extended

# build lint image
docker build -t "$LINT_IMAGE" - << 'DOCKERFILE'
FROM rust:1.86

RUN rustup component add rustfmt clippy && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
      pkg-config libssl-dev libwebp-dev git curl && \
    cargo install cargo-deny cargo-outdated cargo-about && \
    rm -rf /var/lib/apt/lists/*
DOCKERFILE

# run lint checks
docker run --rm   -v "$PWD":/usr/src/app   -w /usr/src/app   "$LINT_IMAGE" bash -c "
    # フォーマット・型チェック・Clippy
    cargo fmt --all &&
    cargo check &&
    cargo clippy -- -D warnings &&

    cargo-about init
    # cargo-about で NOTICE を自動生成（テンプレート about.hbs がある前提）
    cargo-about generate --output-file NOTICE

    cargo outdated || true
  "

# build app image for vulnerability scan
APP_IMAGE="tororomeshi/re_excel"
docker build -t "${APP_IMAGE}:lint-temp" .

# scan with Trivy using official Trivy コンテナ
docker run --rm   -v /var/run/docker.sock:/var/run/docker.sock   -v "${HOME}/.cache/trivy":/root/.cache/trivy   aquasec/trivy:latest image     --exit-code 1     --severity CRITICAL,HIGH     "${APP_IMAGE}:lint-temp"

# cleanup scan image
docker rmi "${APP_IMAGE}:lint-temp" || true
