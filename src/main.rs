use kaisen_yao_sqlite::{extract, query, transform_load};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    if fs::create_dir_all("data").is_err() {
        eprintln!("Error creating data directory.");
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            extract(
                "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/births/US_births_2000-2014_SSA.csv?raw=true",
                "data/US_births.csv",
                "data",
            );
        }
        "transform_load" => match transform_load("data/US_births.csv") {
            Ok(_) => println!("Data loaded successfully!"),
            Err(err) => eprintln!("Error during transform and load: {:?}", err),
        },
        "query" => {
            if let Some(q) = args.get(2) {
                match query(q) {
                    Ok(_) => println!("Query executed successfully!"),
                    Err(err) => eprintln!("Error executing query: {:?}", err),
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
        }
    }
}