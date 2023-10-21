extern crate psutil;

use std::time::Instant;
use std::process;
use psutil::memory;
use std::io;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "python_query_log.md";

fn log_query(message: &str, time: f64, memory_used: f64) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
    {
        writeln!(file, "\nThe action is {}\n\n\n", message).expect("Error writing to log file");
        writeln!(file, "Elapsed time: {} microseconds\n\n\n", time).expect("Error writing to log file");
        writeln!(file, "- Memory used: {} kB\n", memory_used).expect("Error writing to log file");
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let memory_before = memory::virtual_memory()?.used as f64 / 1024.0;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return Ok(());
    }

    let action = &args[1];
    if action == "extract" {
        // Extract action
        extract(
            "https://raw.githubusercontent.com/fivethirtyeight/data/master/goose/goose_rawdata.csv?raw=true",
            "data/Goose.csv",
            "data",
        )?;
    } else if action == "transform_load" {
        // Transform and load action
        match transform_load("data/Goose.csv") {
            Ok(_) => println!("Data loaded successfully!"),
            Err(err) => eprintln!("Error: {:?}", err),
        }?;
    } else if action == "query" {
        // Query action
        if args.len() >= 3 {
            if let Err(err) = query(&args[2]) {
                eprintln!("Error: {:?}", err);
            } else {
                println!("Query executed successfully!");
            }
        } else {
            println!("Usage: {} query [SQL query]", args[0]);
        }
    } else {
        println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
    }

    let end_time = start_time.elapsed();
    let elapsed_time_micros = end_time.as_micros() as f64;
    let memory_after = memory::virtual_memory()?.used as f64 / 1024.0;
    let memory_used = memory_after - memory_before;

    log_query(action, elapsed_time_micros, memory_used);

    Ok(())
}
