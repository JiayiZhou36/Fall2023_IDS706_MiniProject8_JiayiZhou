use jiayi_zhou_sqlite::{extract, log_query, query, transform_load};
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let start_time = Instant::now();
    let mem_info_before = sys_info::mem_info().unwrap();

    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            extract(
                "https://raw.githubusercontent.com/fivethirtyeight/data/master/goose/goose_rawdata.csv?raw=true",
                "data/Goose.csv",
                "data",
            );
        }
        "transform_load" => match transform_load("data/Goose.csv") {
            Ok(_) => println!("Data loaded successfully!"),
            Err(err) => eprintln!("Error: {:?}", err),
        },
        "query" => {
            if let Some(q) = args.get(2) {
                if let Err(err) = query(q) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                    let end_time = Instant::now();
                    let elapsed_time = end_time.duration_since(start_time);
                    let mem_info_after = sys_info::mem_info().unwrap();
                    let mem_used = mem_info_after.total - mem_info_before.total;

                    match log_query(&q, &elapsed_time.as_micros(), &mem_used) {
                        Ok(_) => {}
                        Err(e) => println!("Error: {:?}", e),
                    }
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
