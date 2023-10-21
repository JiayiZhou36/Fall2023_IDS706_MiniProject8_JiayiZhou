"""Import Libraries"""
import sys
import time
import psutil
from mylib.lib import extract, transform_load, query, log_query


def main():
    start_time = time.perf_counter()
    memory_before = psutil.virtual_memory().used / (1024.0)

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
        end_time = time.perf_counter()
        elapsed_time_micros = (end_time - start_time) * 1e6
        memory_after = psutil.virtual_memory().used / (1024.0)
        memory_used = memory_after - memory_before

    elif action == "transform_load":
        result = transform_load("data/Goose.csv")
        if isinstance(result, str):
            print("Data loaded successfully!")
        else:
            print(f"Error: {result}")
        end_time = time.perf_counter()
        elapsed_time_micros = (end_time - start_time) * 1e6
        memory_after = psutil.virtual_memory().used / (1024.0)
        memory_used = memory_after - memory_before

    elif action == "query":
        if len(args) >= 3:
            query_result = query(args[2])
            if query_result is None:
                print("Query executed successfully!")
            else:
                print(f"Error: {query_result}")
        else:
            print(f"Usage: {args[0]} query [SQL query]")
        end_time = time.perf_counter()
        elapsed_time_micros = (end_time - start_time) * 1e6
        memory_after = psutil.virtual_memory().used / (1024.0)
        memory_used = memory_after - memory_before

        log_query(
            args[2],
            elapsed_time_micros,
            memory_used,
        )
    else:
        print("Invalid action. Use 'extract', 'transform_load', or 'query'.")


if __name__ == "__main__":
    main()
