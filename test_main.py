"""Test main.py functions"""
from mylib.lib import extract, query, transform_load
import os


def test_extract():
    """Test extract function"""
    url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/goose/goose_rawdata.csv?raw=true"
    file_path = "data/Goose.csv"
    directory = "data"

    extract(url, file_path, directory)

    assert os.path.exists(file_path)


def test_transform_load():
    """Test transform_load function"""
    dataset = "data/Goose.csv"
    result = transform_load(dataset)

    assert result == "Goose.db"


def test_query():
    """Test query function"""
    # Execute a SELECT query
    select_query = "SELECT * FROM Goose WHERE name = 'Emma Watson';"
    result = query(select_query)

    assert result is None


if __name__ == "__main__":
    test_extract()
    test_transform_load()
    test_query()
