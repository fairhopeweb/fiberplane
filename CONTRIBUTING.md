# Contributing

Please be advised that even though many of our repositories are open for outside
contributions, `fiberplane-rs` is primarily a **read-only** repository for our
community. Fiberplane uses this repository to develop new features out in the
open, and you are encouraged to build custom solution on top of the source code
we provide here. Our [issue tracker](https://github.com/fiberplane/fiberplane-rs/issues)
and [discussions forum](https://github.com/fiberplane/fiberplane-rs/discussions)
are open to all in case you have issues or questions/suggestions.

Creating pull requests is restricted to Fiberplane employees, because changes to
the core data structures in this repository also need to be reflected in our
closed-source components. As such, _the following instructions are intended for
Fiberplane employees only._

# Set up Artifactory

Crates from this repository are automatically pushed to Artifactory upon merging
to main.

1. Login to [Artifactory](https://fiberplane.jfrog.io/ui/login/) using your
   Fiberplane email address.
2. Go to your [user profile](https://fiberplane.jfrog.io/ui/user_profile) and
   create an _Identity Token_.
3. Copy the generated token and paste it into a file on your workstation called
   `~/.cargo/credentials.toml`. The file should look like this:

```toml
[registries.artifactory]
token = "Bearer <TOKEN>"
```
