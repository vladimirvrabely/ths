set shell := ["fish", "-c"]

default:
        @just --list

# Rust fmt
fmt:
        cargo +nightly fmt

# Rust clippy
clippy: fmt
        cargo clippy

# Rust check
check: clippy
        cargo check

# Rust build
build: clippy
        cargo build

# Rust doc
doc:
        RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --workspace --no-deps --all-features

# Run
run:
        RUST_LOG=info,ths=debug cargo run --bin ths

