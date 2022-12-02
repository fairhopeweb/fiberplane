#!/bin/bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rm -r "$SCRIPT_DIR/../fiberplane-api-client"

docker run --rm \
    -v "$(dirname $SCRIPT_DIR):/local" \
    -u "$(id -u ${USER}):$(id -g ${USER})" \
    openapitools/openapi-generator-cli:v5.2.1 \
        generate \
            -i /local/schemas/openapi_v1.yml \
            -p packageName=fiberplane-api-client \
            -g rust \
            -o /local/fiberplane-api-client \
            --skip-validate-spec

cd "$SCRIPT_DIR/../fiberplane-api-client"
cargo fmt

# Git patches don't apply if we're not in the base directory of the project (where .git lives): https://stackoverflow.com/a/67790361/11494565
cd "$SCRIPT_DIR/../"
git apply -v ./schemas/patches/*.patch
