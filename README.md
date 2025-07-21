# Uxn11 Rust Hook Template

This repo demonstrates how to hook into Uxn-like device output (DEO) ports from C into Rust via FFI.

## Features

- Global and per-port DEO hooks from Rust
- FFI-safe
- No unsafe code on Rust side

## Usage

```bash
cargo run
```

Expected output:
```
(Rust) Global hook: port=0x18, value=0x48
(Rust) Per-port hook: port=0x18, value=0x48
(C) emu_deo: port=0x18, value=0x48
...
```
