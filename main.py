import sys
from mylib.lib import extract, transform_load, query


def main():
    args = sys.argv
    if len(args) < 2:
        print(f"Usage: {args[0]} [action]")
        return

    action = args[1]
    if action == "extract":
        extract(
            "https://raw.githubusercontent.com/fivethirtyeight/data/master/goose/goose_rawdata.csv?raw=true",
            "data/Goose.csv",
            "data",
        )
    elif action == "transform_load":
        result = transform_load("data/Goose.csv")
        if isinstance(result, str):
            print("Data loaded successfully!")
        else:
            print(f"Error: {result}")
    elif action == "query":
        if len(args) >= 3:
            query_result = query(args[2])
            if query_result is None:
                print("Query executed successfully!")
            else:
                print(f"Error: {query_result}")
        else:
            print(f"Usage: {args[0]} query [SQL query]")
    else:
        print("Invalid action. Use 'extract', 'transform_load', or 'query'.")


if __name__ == "__main__":
    main()
