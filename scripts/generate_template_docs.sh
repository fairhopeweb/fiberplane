TEMPLATES_DIR="fp-templates"
if [ -x "$(command -v npx)" ]; then
  npx -y jsdoc-to-markdown -c "$TEMPLATES_DIR/jsdoc.json" "$TEMPLATES_DIR/fiberplane.libsonnet" > "$TEMPLATES_DIR/docs/template_api.md"
else
  docker run --rm -v "`realpath $TEMPLATES_DIR`:/$TEMPLATES_DIR" node:17 npx -y jsdoc-to-markdown -c "/$TEMPLATES_DIR/jsdoc.json" "/$TEMPLATES_DIR/fiberplane.libsonnet" > "$TEMPLATES_DIR/docs/template_api.md"
fi
