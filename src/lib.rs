use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "rust_query_log.md";

pub fn log_query(message: &str, time: &u128, memory_used: &u64) -> Result<()> {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_FILE) {
        writeln!(file, "\nThe query is {}\n", message).expect("Error writing to log file");
        writeln!(file, "Elapsed time: {} microseconds\n", time).expect("Error writing to log file");
        writeln!(file, " Memory used: {} kB\n", memory_used).expect("Error writing to log file");
    } else {
        eprintln!("Error opening log file for writing.");
    }
    Ok(())
}

pub fn extract(url: &str, file_path: &str, directory: &str) {
    if fs::metadata(directory).is_err() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }

    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = fs::File::create(file_path).expect("Failed to create file");

    std::io::copy(&mut response, &mut file).expect("Failed to copy content");

    println!("Extraction successful!");
}

pub fn transform_load(dataset: &str) -> Result<String> {
    let conn = Connection::open("Goose.db")?;

    conn.execute("DROP TABLE IF EXISTS Goose", [])?;

    conn.execute(
        "CREATE TABLE Goose (
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
        )",
        [],
    )?;

    let mut rdr = csv::Reader::from_path(dataset).expect("Failed to read dataset");

    let mut stmt = conn.prepare(
        "INSERT INTO Goose (
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
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute([
                    &record[0],
                    &record[1],
                    &record[2],
                    &record[3],
                    &record[4],
                    &record[5],
                    &record[6],
                    &record[7],
                    &record[8],
                    &record[9],
                    &record[10],
                    &record[11],
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok("Goose.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("Goose.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, i32>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, String>(4)?,
                row.get::<usize, i32>(5)?,
                row.get::<usize, i32>(6)?,
                row.get::<usize, i32>(7)?,
                row.get::<usize, f32>(8)?,
                row.get::<usize, f32>(9)?,
                row.get::<usize, f32>(10)?,
                row.get::<usize, f32>(11)?,
                row.get::<usize, String>(12)?,
            ))
        })?;

        for result in results {
            match result {
                Ok((
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
                )) => {
                    println!(
                        "Result: id={}, name={}, year={}, team={}, league={}, goose_eggs={}, broken_eggs={}, mehs={}, league_average_gpct={}, ppf={}, replacement_gpct={}, gwar={}, key_retro={}",
                        id, name, year, team, league, goose_eggs, broken_eggs, mehs, league_average_gpct, ppf, replacement_gpct, gwar, key_retro
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    Ok(())
}
