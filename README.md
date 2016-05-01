# Rust Runnables

[![Build Status](https://travis-ci.org/azriel91/rust_runnables.svg?branch=master)](https://travis-ci.org/azriel91/rust_runnables) [![Crates.io](https://img.shields.io/crates/v/rust_runnables.svg)](https://crates.io/crates/rust_runnables)

Rust traits that mimic the `Runnable` and `Callable<T>` interfaces in Java.

## Usage

Please see [`src/lib.rs`](src/lib.rs) for code examples.

## Building

1. Make sure you have [Rust](https://www.rust-lang.org/) (stable) and Cargo installed
2. Run the following command:

    ```bash
    cargo build && cargo test
    ```

## Releasing

1. Make sure the [last build](https://travis-ci.org/azriel91/rust_runnables) is successful
2. Tag the commit

    ```bash
    git tag 0.0.1  # see Cargo.toml
    ```

3. Publish the package

    Make sure you have [publishing](http://doc.crates.io/crates-io.html#publishing-crates) permissions, and have [logged in](https://crates.io/me) to Cargo, then run:

    ```
    cargo publish
    ```

4. Update `Cargo.toml` to the next version

## License

Licensed under [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
