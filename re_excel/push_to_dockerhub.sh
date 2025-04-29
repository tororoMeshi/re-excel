#!/usr/bin/env bash
set -euo pipefail

# DockerHub ユーザー名（環境変数で上書き可）
DOCKERHUB_USER="${DOCKERHUB_USER:-tororomeshi}"

# default tag: timestamp
IMAGE_TAG=$(date +%Y%m%d%H%M)
if [ $# -ge 1 ]; then IMAGE_TAG="$1"; fi

IMAGE_NAME="${DOCKERHUB_USER}/re_excel"

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
cd "$SCRIPT_DIR"

# remove Cargo.lock to delegate deps to Docker
if [ -f "Cargo.lock" ]; then
  echo "Removing Cargo.lock..."
  rm Cargo.lock
fi

echo "Building Docker image..."
docker build -t "${IMAGE_NAME}:${IMAGE_TAG}" -t "${IMAGE_NAME}:latest" .

push_image() {
  local TAG="$1"
  echo "Pushing ${IMAGE_NAME}:${TAG}..."
  if ! docker push "${IMAGE_NAME}:${TAG}"; then
    echo "Docker push failed for tag ${TAG}. Please run 'docker login'." >&2
    exit 1
  fi
}

push_image "${IMAGE_TAG}"
push_image "latest"

echo "✅ Docker image pushed: ${IMAGE_NAME}:${IMAGE_TAG} (also tagged latest)"
