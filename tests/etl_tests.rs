use jiayi_zhou_sqlite::{extract, query, transform_load};

#[test]
fn test_extract() {
    let url =
        "https://raw.githubusercontent.com/fivethirtyeight/data/master/goose/goose_rawdata.csv?raw=true";
    let file_path = "data/Goose.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/Goose.csv";
    let result = transform_load(dataset);

    assert_eq!(result.unwrap(), "Goose.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM Goose WHERE name = 'Emma Watson';";
    let result = query(select_query);

    assert!(result.is_ok());
}
