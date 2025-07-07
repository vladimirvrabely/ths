set shell := ["fish", "-c"]

default:
        @just --list

# Rust fmt
fmt:
        cargo +nightly fmt

# Rust clippy
clippy: fmt
        cargo clippy --workspace

# Rust check
check: clippy
        cargo check --workspace

# Rust build
build: clippy
        cargo build --workspace

# Rust doc
doc:
        RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --workspace --no-deps --all-features

# Run
run:
        RUST_LOG=info,ths=debug cargo run --bin ths


cross-build:
        cross build --release --target=aarch64-unknown-linux-gnu

