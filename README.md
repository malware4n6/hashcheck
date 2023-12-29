# Some Rust project

The main objective of the project was to discover Rust, so:

**!!! DO NOT TAKE THIS PROJECT AS AN EXAMPLE !!!**

# Developer

* [install Cargo and Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* install vscod(e|ium) and the following plugins
    * WSL (ms-vscode-remote.remote-wsl) -- way not work with `codium`
    * rust-analyzer (rust-lang.rust-analyzer)
* in WSL: `sudo apt install rust-src` to get autocomplete feature


```sh
# generate the project
cargo new hashchecker
# add some libs
cargo add log
cargo add env_logger
# add other libs here
# in case of error
# cargo remove io_tee

# build only
cargo build --release

# run directly after building
cargo run --release foo bar

# later...
cargo update

# run binaries
RUST_LOG=trace ./target/release/hashcheck foo bar
```

## Documentation

* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)