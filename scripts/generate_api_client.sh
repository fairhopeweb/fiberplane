#!/usr/bin/env bash

# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

rm -rf "$SCRIPT_DIR/../fiberplane-api-client" || true

if ! command -v fp-openapi-rust-gen &>/dev/null; then
  # not in path; use docker image
  echo "pulling newest docker image for our openapi generator and running it"
  echo "if this fails, please ensure you have executed 'docker login' with the 'fiberplane' account (creds in 1password)"

  docker run --rm --pull=always \
    -v "$(dirname $SCRIPT_DIR):/local" \
    -u "$(id -u ${USER}):$(id -g ${USER})" \
    fiberplane/fp-openapi-rust-gen:latest \
    --output /local/fiberplane-api-client \
    /local/schemas/openapi_v1.yml \
    --local
else
  # use the one from PATH if its already there
  fp-openapi-rust-gen --output fiberplane-api-client "$SCRIPT_DIR/../schemas/openapi_v1.yml" --local
fi

cd "$SCRIPT_DIR/../fiberplane-api-client"
cargo fmt -p fiberplane-api-client
