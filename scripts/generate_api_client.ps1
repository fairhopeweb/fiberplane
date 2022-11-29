# Windows version of the `generate_api_client.sh` script

$start_dir = Get-Location;
$script_path = ($PSScriptRoot);
$root_dir = (Get-Item $script_path ).parent.FullName;
$api_client_dir = Join-Path $root_dir "fp-api-client";

if (Test-Path $api_client_dir) {
    Remove-Item $api_client_dir -Recurse
}

docker run --rm `
    -v "${root_dir}:/local" `
    openapitools/openapi-generator-cli:v5.2.1 `
        generate `
            -i /local/schemas/openapi_v1.yml `
            -p packageName=fp-api-client `
            -g rust `
            -o /local/fp-api-client `
            --skip-validate-spec

Set-Location $api_client_dir
cargo fmt

# Git patches don't apply if we're not in the base directory of the project (where .git lives): https://stackoverflow.com/a/67790361/11494565
Set-Location $root_dir
$files = Get-ChildItem "schemas\patches\*.patch"

foreach ($file in $files) {
    git apply -v $file
}

# At the end of the script, get back to the directory that we started in
Set-Location $start_dir
