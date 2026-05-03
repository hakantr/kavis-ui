#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOCKFILE="${1:-$ROOT_DIR/Cargo.lock}"

if [[ ! -f "$LOCKFILE" ]]; then
  echo "Cargo.lock not found: $LOCKFILE" >&2
  exit 1
fi

version="$(
  awk '
    /^\[\[package\]\]/ {
      in_package = 1
      package_name = ""
      next
    }

    in_package && /^name = "wasm-bindgen"$/ {
      package_name = "wasm-bindgen"
      next
    }

    in_package && package_name == "wasm-bindgen" && /^version = / {
      gsub(/"/, "", $3)
      print $3
      exit
    }
  ' "$LOCKFILE"
)"

if [[ -z "$version" ]]; then
  echo "Could not determine wasm-bindgen version from $LOCKFILE" >&2
  exit 1
fi

echo "Ensuring wasm-bindgen-cli $version..."
if command -v wasm-bindgen >/dev/null 2>&1; then
  installed_output="$(wasm-bindgen --version 2>/dev/null || true)"
  installed_version="$(awk '{print $2}' <<<"$installed_output")"

  if [[ "$installed_version" == "$version" ]]; then
    echo "wasm-bindgen-cli $version is already installed."
    exit 0
  fi

  if [[ -n "$installed_version" ]]; then
    echo "Found wasm-bindgen-cli $installed_version; installing $version instead."
  fi
fi

cargo install --force wasm-bindgen-cli --version "$version" --locked
