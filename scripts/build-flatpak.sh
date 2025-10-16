#!/usr/bin/env bash
set -euo pipefail
MANIFEST=flatpak/com.example.CosmicLogoApplet.json
BUILD_DIR=build-dir
REPO_DIR=repo

if ! command -v flatpak-builder >/dev/null 2>&1; then
  echo "flatpak-builder not found" >&2
  exit 1
fi

flatpak-builder "$BUILD_DIR" "$MANIFEST" --force-clean --install --user
flatpak build-export "$REPO_DIR" "$BUILD_DIR"
flatpak build-bundle "$REPO_DIR" desktop-logo-applet.flatpak com.example.CosmicLogoApplet

echo "Bundle created: desktop-logo-applet.flatpak"