# Provider Protocol

The provider protocol defines the two-way bindings between Fiberplane providers and their hosts.
[`fp-bindgen`](https://github.com/fiberplane/fp-bindgen) is used to generate the host and guest binding code from the protocol.

## Generating Bindings

To regenerate the bindings from the protocol, run:

```sh
cargo run && cargo fmt
```
