#!/bin/bash
# 使用例: ./new-rust.sh my-project-name
set -eux

if [ $# -ne 1 ]; then
  echo "Usage: $0 <project-name>" >&2
  exit 1
fi

PROJECT_NAME="$1"
HOST_DIR="$(pwd)/$PROJECT_NAME"

# プロジェクト用ディレクトリを作成
mkdir -p "$HOST_DIR"

# Dockerで cargo new を実行（--vcsなし、MITライセンス）
docker run --rm \
  -v "$HOST_DIR":/opt \
  -w /opt \
  rust:1.86 \
  bash -c "cargo new . --vcs none --license MIT --name $PROJECT_NAME"
