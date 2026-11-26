#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
mkdir -p .nsh
rustc --edition=2024 scripts/generate-tsx.rs -o .nsh/generate-tsx
.nsh/generate-tsx
if [ "$#" -eq 0 ]; then
    set -- package --target web --out dist
fi
exec nsh "$@"
