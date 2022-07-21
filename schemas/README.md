# Patching

Occasionally the OpenAPI generator for rust will make code that doesn't compile.
Ideally the generator should be fixed but ain't nobody got time for dat so
instead we apply git patches to the generated and formatted code.
Here's the workflow:

1. Edit `openapi_v1.yml` to with your changes
1. Run `{git_root}/scripts/generate_api_client.sh` (or `.ps1` on windows)
1. Commit the changes to the schema and api_client (this important for a clean diff)
1. Build the api_client: `cargo build -p fp-api-client` if it doesn't compile we need to make a patch :(
1. Make your changes in the `api_client/` folder. Most likely `api_client/src/apis/default_api.rs`
1. Create a patch of the changes: `git diff -p api_client/ > schemas/patches/your_patch_name.patch`
1. Run `{git_root}/scripts/generate_api_client.sh` again and test it applies the patch and now compiles
1. Commit everything and make PR
1. Great success

### PSA for WSL and maybe windows:
Git can be very picky about `CRLF` so make sure the patches are using `LF` and not `CRLF` on WSL
