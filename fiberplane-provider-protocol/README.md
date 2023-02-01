# Provider Protocol

> Three-way bindings between Fiberplane Providers and their hosts.

**_Are you looking for examples of providers? Head over to our
[Providers Repo](https://github.com/fiberplane/providers)._**

This crate contains the low-level definition of our provider protocol, and the
bindings we generate from it.

Note that we do not publish the provider protocol itself to
[Crates.io](https://crates.io). Instead, we publish the
[provider bindings](fiberplane-provider-bindings/) and
[runtime bindings](fiberplane-provider-runtime/).

We also generate a [TypeScript runtime](ts-runtime/) that we use inside
[Fiberplane Studio](https://studio.fiberplane.com).

We use [`fp-bindgen`](https://github.com/fiberplane/fp-bindgen) to generate the
host and guest binding code from the protocol.

## Generating Bindings

To regenerate the bindings from the protocol, run:

```sh
cargo run && cargo fmt
```
