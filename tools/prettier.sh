#!/usr/bin/env bash
set -euo pipefail

# Run Prettier directly in the workspace root
WORKSPACE_ROOT="${BUILD_WORKSPACE_DIRECTORY:-$(pwd)}"

# Prettier will follow real files in the workspace
npx prettier "$@" "$WORKSPACE_ROOT/README.md" "$WORKSPACE_ROOT/CONTRIBUTING.md" "$WORKSPACE_ROOT/**/*.{md,js,json,yml,yaml,css,ts,tsx}"
