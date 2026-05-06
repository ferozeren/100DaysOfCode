from __future__ import annotations

from pathlib import Path

# from py_rust_web_pipeline import get_csv
import polars as pl

# Conigurations
pl.Config.set_tbl_cols(-1)
pl.Config.set_tbl_rows(-1)

pl.Config.set_fmt_str_lengths(100)  # Default 30


def extract_csv(csv: str) -> pl.DataFrame:
    csv_path: Path = Path(csv)
    try:
        data: pl.DataFrame = pl.read_csv(csv_path)
    except FileNotFoundError as exc:
        raise FileNotFoundError(exc) from exc
    return data


def display_csv(csv_data: pl.DataFrame, rows: None | int = None) -> None:
    if rows and rows <= len(csv_data):
        print(len(csv_data))
        print(csv_data.head(rows))
    else:
        print(csv_data)


def run() -> None:
    try:
        csv: pl.DataFrame = extract_csv("data/Teen_Mental_Health_Dataset.csv")
        display_csv(csv, 100)
    except Exception as exc:
        print(f"Error: {exc} ")


def main() -> None:
    run()


if __name__ == "__main__":
    main()
