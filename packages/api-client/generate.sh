#!/usr/bin/env bash
# Generate TypeScript API client from OpenAPI spec
# Option A: Requires running backend on port 3068
# Option B: Use export_openapi binary (no server needed)
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OPENAPI_URL="${OPENAPI_URL:-http://localhost:3068/openapi.json}"
OPENAPI_JSON="$SCRIPT_DIR/openapi.json"

echo "Fetching OpenAPI spec from $OPENAPI_URL..."
curl -sf "$OPENAPI_URL" -o "$OPENAPI_JSON"

echo "Generating TypeScript client..."
cd "$SCRIPT_DIR"
pnpm dlx @hey-api/openapi-ts \
  --input "$OPENAPI_JSON" \
  --output src/ \
  --client fetch

echo "Done! Generated files in packages/api-client/src/"
