# DOOM Rust

This is a very early work in progress project to write the DOOM engine in rust.

Right now it just reads the vertexes and lines for E1M1 and draws them.

### Building

This requires **rustc v1.47.0**. There is a problem in one of the dependencies that makes it fail with rustc v1.48+

1. Set the correct rust toolchain
```bash
$ rustup override set 1.47.0
```

2. Copy the wad file to the root of the project. You can find the shareware wad for instance [here](http://distro.ibiblio.org/pub/linux/distributions/slitaz/sources/packages/d/doom1.wad)

2. Run the project
```bash
$ cargo run
```