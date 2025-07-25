# Solana Bytes Reader

A lightweight, ergonomic utility library for safely and efficiently reading bytes slices, primarily designed for Solana program development.

This crate provides basic primitives for reading various integer types, booleans, and fixed-size byte arrays from raw byte slices with offset tracking.

## Features

- Safe, offset-aware reading of primitive types (`u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`).
- Boolean reading with validation.
- Support for reading fixed-size byte arrays.
- Optional zero-copy deserialization for Pod types via the `bytemuck` feature.
- Traits for reading (`ReadBytes`) and peeking (`PeekIntoBytes`) without advancing the offset.

## Usage

Add to your `Cargo.toml`:

```toml
# Enables basic features (by default)
solana-bytes-reader = "0.2.0"

# Enables `bytemuck` feature (disabled by default)
solana-bytes-reader = { version = "0.2.0", features = ["bytemuck"] }
```

## Example

Initialize a `Reader` with a byte slice and use the provided methods to parse data sequentially or peek into upcoming bytes without consuming them.

```rust
use solana_bytes_reader::{Reader, ReadBytes};

let data: &[u8] = /* your byte slice */;
let mut reader: Reader = Reader::new(data);

let value: u32 = reader.read_u32()?;
let flag: bool = reader.read_bool()?;
// ...
```