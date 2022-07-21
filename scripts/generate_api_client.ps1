# Windows version of the `generate_api_client.sh` script

$start_dir = Get-Location;
$script_path = ($PSScriptRoot);
$root_dir = (Get-Item $script_path ).parent.FullName;

docker run --rm `
    -v "${root_dir}:/local" `
    openapitools/openapi-generator-cli:v5.2.1 `
        generate `
            -i /local/schemas/openapi_v1.yml `
            -p packageName=fp-api-client `
            -g rust `
            -o /local/api_client `
            --skip-validate-spec

$api_client_dir = Join-Path $root_dir "api_client";
Set-Location $api_client_dir
cargo fmt

# Git patches don't apply if we're not in the base directory of the project (where .git lives): https://stackoverflow.com/a/67790361/11494565
Set-Location $root_dir
git apply -v .\schemas\patches\*.patch

# At the end of the script, get back to the directory that we started in
Set-Location $start_dir
