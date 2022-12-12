# Windows version of the `generate_api_client.sh` script

$start_dir = Get-Location;
$script_path = ($PSScriptRoot);
$root_dir = (Get-Item $script_path).parent.FullName;
$api_client_dir = Join-Path $root_dir "fiberplane-api-client";

if (Test-Path $api_client_dir)
{
    Remove-Item $api_client_dir -Recurse
}

if ($null -eq (Get-Command "fp-openapi-rust-gen.exe" -ErrorAction SilentlyContinue))
{
    # not in path; use docker image
    docker.exe run --rm `
        -v "${root_dir}:/local" `
            fiberplane/fp-openapi-rust-gen:latest `
            --output /local/fiberplane-api-client `
            /local/schemas/openapi_v1.yml `
            --local
}
else
{
    # use the one from PATH if its already there
    fp-openapi-rust-gen.exe --output fiberplane-api-client .\schemas\openapi_v1.yml --force --local
}

Set-Location $api_client_dir
cargo fmt

# go back to the beginning so our user doesn't get confused as they're in a different directory now
Set-Location $start_dir
