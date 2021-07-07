strip\_cargo\_version
=====================

A small hacky utility for setting the version of a Rust crate to `0.0.0`.

I use this for optimising build-times in Docker, avoiding
having to recompile dependencies when bumping the crate version.

It currently works by simply setting `package.version` in Cargo.toml, as well as setting
`package.<name>.version` in Cargo.lock (where `<name>` is the `package.name` value in Cargo.toml).

If the Cargo.toml contains a workspace, it will strip `workspace.members` as well.

