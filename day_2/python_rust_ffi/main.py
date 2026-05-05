from python_rust_ffi import compute_run, sum_as_string


def main() -> None:
    num1: int = 10
    num2: int = 11
    result: str = sum_as_string(10, 11)
    print(f"{num1} + {num2} = {result}")

    # Display `Computing..` message to the user
    compute_run()

    pass


if __name__ == "__main__":
    main()
