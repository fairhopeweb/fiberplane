# Provider Protocol

> Three-way bindings between Fiberplane Providers and their hosts.

[`fp-bindgen`](https://github.com/fiberplane/fp-bindgen) is used to generate the
host and guest binding code from the protocol.

Note that we do not publish the provider protocol itself to
[Crates.io](https://crates.io). Instead, we publish the
[provider bindings](fiberplane-provider-bindings/) and
[runtime bindings](fiberplane-provider-runtime/).

We also generate a [TypeScript runtime](ts-runtime/) that we use inside
[Fiberplane Studio](https://studio.fiberplane.com).

## Generating Bindings

To regenerate the bindings from the protocol, run:

```sh
cargo run && cargo fmt
```
