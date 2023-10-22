"""Library for the goose project"""
import os
import sqlite3
import csv
import requests

LOG_FILE = "python_query_log.md"


def log_query(message, time, memory_used):
    """adds to a query markdown file"""
    with open(LOG_FILE, "a") as file:
        file.write(f"\nThe query is {message}\n\n\n")
        file.write(f"Elapsed time: {time} microseconds\n\n\n")
        file.write(f"- Memory used: {memory_used} kB\n")


def extract(url, file_path, directory):
    """Extract a url to a file path"""
    if not os.path.exists(directory):
        os.makedirs(directory)
    with requests.get(url) as r:
        with open(file_path, "wb") as f:
            f.write(r.content)
    return file_path


def transform_load(dataset):
    "Transform and load a dataset"
    conn = sqlite3.connect("Goose.db")
    cursor = conn.cursor()

    cursor.execute("DROP TABLE IF EXISTS Goose")
    cursor.execute(
        """CREATE TABLE Goose (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            year INTEGER,
            team TEXT,
            league TEXT,
            goose_eggs INTEGER,
            broken_eggs INTEGER,
            mehs INTEGER,
            league_average_gpct REAL,
            ppf REAL,
            replacement_gpct REAL,
            gwar REAL,
            key_retro TEXT
        )"""
    )

    with open(dataset, "r") as csvfile:
        csvreader = csv.reader(csvfile)
        next(csvreader)  # Skip the header row

        for row in csvreader:
            cursor.execute(
                """
        INSERT INTO Goose (
            name,
            year,
            team,
            league,
            goose_eggs,
            broken_eggs,
            mehs,
            league_average_gpct,
            ppf,
            replacement_gpct,
            gwar,
            key_retro
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)""",
                row,
            )

    conn.commit()
    conn.close()
    return "Goose.db"


def query(query):
    """runs a query a user inputs"""
    # Connect to the SQLite database
    conn = sqlite3.connect("Goose.db")

    # Create a cursor object to execute SQL queries
    cursor = conn.cursor()

    # Execute the query
    cursor.execute(query)

    # If the query modifies the database, commit the changes
    if (
        query.strip().lower().startswith("insert")
        or query.strip().lower().startswith("update")
        or query.strip().lower().startswith("delete")
    ):
        conn.commit()

    # Close the cursor and connection
    cursor.close()
    conn.close()
    return "succcess"
