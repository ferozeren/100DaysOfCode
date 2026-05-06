import pandas as pd
import streamlit


def main() -> None:
    df = pd.DataFrame(
        {
            "name": ["john", "alex", "simon", "raj", "anna"],
            "age": [19, 29, 23, 20, 27],
            "gender": ["m", "m", "m", "m", "f"],
        }
    )
    print("Data Frame:\n", df)

    streamlit.dataframe(df)


if __name__ == "__main__":
    main()
