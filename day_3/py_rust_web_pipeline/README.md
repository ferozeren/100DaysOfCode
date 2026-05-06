# Python Rust Data Pipeline & Dashboard (WIP)

Note: **This project is currently in the active building stage. Many backend modules are incomplete, and architectural features are subject to change.**

This project is a hybrid data processing pipeline and web dashboard.

- Dual-Engine Processing: It processes CSV datasets. If the dataset is small or medium, it processes it directly in Python using `Polars`.
- Rust Acceleration: If the dataset is large, it offloads the heavy processing to a high-performance Rust backend before returning the clean data to Python.
- Web Visualization: Finally, the data is displayed on a `Streamlit `web dashboard.

Current Build Progress:

- [x] Data Ingestion: Initial `Polars` CSV loading and configuration.
- [x] Backend Infrastructure: Rust/`PyO3` bridge setup for native extensions.
- [ ] Data Cleaning: Comprehensive handling of nulls, schema enforcement, and type casting.
- [ ] Data Analysis: Statistical extraction, grouping, and insight generation.
- [ ] Visualization Layer: Interactive `Streamlit` UI and dashboard components.
