# Patching

Occasionally the OpenAPI generator for rust will make code that doesn't compile.
Ideally the generator should be fixed but ain't nobody got time for dat so
instead we apply git patches to the generated and formatted code.
Here's the workflow:

1. Edit `openapi_v1.yml` to with your changes
2. Run `{git_root}/scripts/generate_api_client.sh` (or `.ps1` on windows)
3. Commit the changes to the `schema` and `fp-api-client`
   - This important for a clean diff
4. Build the api_client: `cargo build -p fp-api-client`
   - **If it doesn't compile we need to make a patch 😔:**
     1. Make your changes in the `fp-api-client/` folder. Most likely `fp-api-client/src/apis/default_api.rs`
     2. Create a patch of the changes: `git diff -p fp-api-client/ > schemas/patches/your_patch_name.patch`
     3. Run `{git_root}/scripts/generate_api_client.sh` again and test it applies the patch and now compiles
5. Commit everything and make PR
6. Great success

### PSA for WSL and maybe windows:
Git can be very picky about `CRLF` so make sure the patches are using `LF` and not `CRLF` on WSL

Configure Git to do this automatically:

```
git config --global core.autocrlf false
```
