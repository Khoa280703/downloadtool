#!/usr/bin/env bash
# Build extension: compile TS, copy static assets, produce 2 zips (Firefox + Edge)
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UNPACKED="$DIR/dist/unpacked"
DIST="$DIR/dist"

echo "Building extension..."

# Step 1: Compile all 3 IIFE entries
pnpm vite build
pnpm vite build --config vite.background.config.ts
pnpm vite build --config vite.popup.config.ts

# Step 2: Copy static assets to dist/unpacked/
mkdir -p "$UNPACKED/popup" "$UNPACKED/icons"

cp "$DIR/src/popup/popup.html" "$UNPACKED/popup/popup.html"
cp "$DIR/src/popup/popup.css"  "$UNPACKED/popup/popup.css"

# Copy icons if they exist (create placeholders if not)
if [ -d "$DIR/icons" ] && ls "$DIR/icons/"*.png 1>/dev/null 2>&1; then
  cp "$DIR/icons/"*.png "$UNPACKED/icons/"
else
  echo "WARNING: No icon PNGs found in icons/ — add icon-16.png, icon-48.png, icon-128.png"
fi

# Step 3: Firefox zip — uses manifest-firefox.json
FIREFOX_STAGING="$DIST/staging-firefox"
rm -rf "$FIREFOX_STAGING" && cp -r "$UNPACKED" "$FIREFOX_STAGING"
cp "$DIR/manifest-firefox.json" "$FIREFOX_STAGING/manifest.json"
cd "$FIREFOX_STAGING" && zip -qr "$DIST/youtube-downloader-firefox.zip" . && cd "$DIR"
rm -rf "$FIREFOX_STAGING"
echo "Built: dist/youtube-downloader-firefox.zip"

# Step 4: Edge zip — uses manifest-edge.json
EDGE_STAGING="$DIST/staging-edge"
rm -rf "$EDGE_STAGING" && cp -r "$UNPACKED" "$EDGE_STAGING"
cp "$DIR/manifest-edge.json" "$EDGE_STAGING/manifest.json"
cd "$EDGE_STAGING" && zip -qr "$DIST/youtube-downloader-edge.zip" . && cd "$DIR"
rm -rf "$EDGE_STAGING"
echo "Built: dist/youtube-downloader-edge.zip"

echo "Done! dist/unpacked/ ready for local testing."
echo "  Firefox: about:debugging → Load Temporary Add-on → dist/unpacked/ (with manifest-firefox.json renamed)"
echo "  Edge:    edge://extensions/ → Developer mode → Load unpacked → dist/unpacked/ (rename manifest-edge.json)"
