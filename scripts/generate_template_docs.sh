#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

# The version of jsdoc-to-markdown to use
VERSION="8.0.0"
TEMPLATES_DIR="fiberplane-templates"

if [ -x "$(command -v npx)" ]; then
  npx -y "jsdoc-to-markdown@${VERSION}" -c "$TEMPLATES_DIR/jsdoc.json" "$TEMPLATES_DIR/fiberplane.libsonnet" --name-format > "$TEMPLATES_DIR/docs/template_api.md"
else
  docker run --rm -v "$(realpath $TEMPLATES_DIR):/$TEMPLATES_DIR" node:18 npx -y "jsdoc-to-markdown@${VERSION}" -c "/$TEMPLATES_DIR/jsdoc.json" --name-format "/$TEMPLATES_DIR/fiberplane.libsonnet" > "$TEMPLATES_DIR/docs/template_api.md"
fi
