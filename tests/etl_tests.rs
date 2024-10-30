use kaisen_yao_sqlite::{extract, query, transform_load};

#[test]
fn test_extract() {
    let url =
        "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/births/US_births_2000-2014_SSA.csv?raw=true";
    let file_path = "data/US_births.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/US_births.csv";
    let result = transform_load(dataset);

    assert_eq!(result.unwrap(), "US_births_DB.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM US_births WHERE id = 1;";
    let result = query(select_query);

    assert!(result.is_ok());
}
