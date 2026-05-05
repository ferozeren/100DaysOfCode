# Python Rust FFI

The goal of this is to run rust(performant) code for computation from the python.

## Prerequisites

- Rust toolchain.
- Python and UV Package Manager.
- `Maturin`, Using any of the following methods:
  ```bash
  uv tool install maturin
  ```
  ```bash
  pip install maturin
  ```
  ```bash
  cargo install maturin
  ```
  You can also use `cargo-binstall` if you have it installed
  ```bash
  cargo binstall maturin
  ```
  Check if `maturin` installed successfully
  ```bash
  maturin --version
  ```
