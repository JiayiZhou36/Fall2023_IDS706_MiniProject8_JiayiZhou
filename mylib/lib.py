"""Library for the goose project"""
import os
import sqlite3
import requests
import csv

LOG_FILE = "python_query_log.md"


def log_query(query):
    """adds to a query markdown file"""
    with open(LOG_FILE, "a") as file:
        file.write(f"```sql\n{query}\n```\n\n")


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
    "Query a database"
    conn = sqlite3.connect("Goose.db")
    cursor = conn.cursor()

    # Read operation
    if query.strip().lower().startswith("select"):
        cursor.execute(query)
        results = cursor.fetchall()
        for result in results:
            (
                id,
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
                key_retro,
            ) = result
            print(
                f"Result: id={id}, name={name}, year={year},"
                f"team={team}, league={league},"
                f"goose_eggs={goose_eggs}, broken_eggs={broken_eggs}, mehs={mehs}, "
                f"league_average_gpct={league_average_gpct}, ppf={ppf}, "
                f"replacement_gpct={replacement_gpct},"
                f"gwar={gwar}, key_retro={key_retro}"
            )

    else:
        # other CUD operations
        cursor.execute(query)

    conn.commit()
    conn.close()
    log_query(query)
    return "Goose.db"


if __name__ == "__main__":
    pass
