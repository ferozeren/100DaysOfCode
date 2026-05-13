# `sqlite_visualize`

A Python module written in Rust using `PyO3` and `rusqlite`, featuring a `Streamlit `dashboard for basic database management.

## Quick Start

Ensure you have `uv`, `rustc`, and just installed.

1.  Sync Dependencies: Set up the virtual environment and update Rust crates.

```
just sync
```

2.  Build Extension: Compile the Rust code into a native Python module.

```
just build
```

3.  Launch Dashboard: Run the `Streamlit` visualization tool.

```
just serve
```

## Commands Overview

- `just sync`: Cleans local pip installs and syncs the `uv` environment.
- `just build`: Compiles Rust logic via `maturin` develop.
- `just run`: Executes the core `main.py` script.
- `just serve`: Starts the `Streamlit` web interface.
- `just deepclean`: nukes `.venv` and all build caches for a fresh start. [Last resort]
