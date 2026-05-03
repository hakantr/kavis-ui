#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCS_DIR="$ROOT_DIR/docs"
GALLERY_DIR="$ROOT_DIR/crates/story-web"
DIST_DIR="$DOCS_DIR/.vitepress/dist"
GALLERY_DIST_DIR="$GALLERY_DIR/www/dist"

echo "==> Dokümantasyon bağımlılıkları kuruluyor"
cd "$DOCS_DIR"
bun install

echo "==> VitePress dokümantasyonu derleniyor"
bun run build

echo "==> WASM galeri derleniyor"
cd "$GALLERY_DIR"
make build-prod

echo "==> Galeri doküman dağıtım klasörüne kopyalanıyor"
rm -rf "$DIST_DIR/gallery"
mkdir -p "$DIST_DIR/gallery"
cp -R "$GALLERY_DIST_DIR"/. "$DIST_DIR/gallery/"

echo "==> Web dağıtımı hazır: $DIST_DIR"
