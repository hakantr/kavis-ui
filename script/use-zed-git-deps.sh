#!/usr/bin/env bash
set -euo pipefail

manifest="${1:-Cargo.toml}"
zed_git="https://github.com/zed-industries/zed"

if [[ -z "${ZED_REV:-}" ]]; then
  echo "ZED_REV ortam değişkeni tanımlı olmalı." >&2
  exit 1
fi

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

awk -v zed_git="$zed_git" -v zed_rev="$ZED_REV" '
function zed_dep(package, features) {
    printf "%s = { git = \"%s\", rev = \"%s\", package = \"%s\"", package, zed_git, zed_rev, package
    if (features != "") {
        printf ", features = %s", features
    }
    print " }"
}

/^\[patch\."https:\/\/github.com\/zed-industries\/zed"\]/ {
    in_zed_patch = 1
    next
}

in_zed_patch && /^\[/ {
    in_zed_patch = 0
}

in_zed_patch {
    next
}

/^gpui = \{ path = "\.\.\/zed\/crates\/gpui" \}/ {
    zed_dep("gpui", "")
    next
}

/^gpui_platform = \{ path = "\.\.\/zed\/crates\/gpui_platform"/ {
    zed_dep("gpui_platform", "[\"font-kit\", \"runtime_shaders\", \"wayland\", \"x11\"]")
    next
}

/^gpui_web = \{ path = "\.\.\/zed\/crates\/gpui_web" \}/ {
    zed_dep("gpui_web", "")
    next
}

/^gpui_macros = \{ path = "\.\.\/zed\/crates\/gpui_macros" \}/ {
    zed_dep("gpui_macros", "")
    next
}

/^reqwest_client = \{ path = "\.\.\/zed\/crates\/reqwest_client" \}/ {
    zed_dep("reqwest_client", "")
    next
}

{
    print
}
' "$manifest" > "$tmp"

mv "$tmp" "$manifest"
trap - EXIT

echo "Zed bağımlılıkları GitHub kaynağına geçirildi: $zed_git@$ZED_REV"
