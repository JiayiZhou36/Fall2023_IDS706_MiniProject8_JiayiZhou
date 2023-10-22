## Fall2023_IDS706 Mini Project 8: Rewrite a Python Script in Rust
### by Jiayi Zhou [![Python CI/CD Pipeline](https://github.com/nogibjj/Fall2023_IDS706_MiniProject8_JiayiZhou/actions/workflows/pythonCI.yml/badge.svg)](https://github.com/nogibjj/Fall2023_IDS706_MiniProject8_JiayiZhou/actions/workflows/pythonCI.yml) [![Rust CI/CD Pipeline](https://github.com/nogibjj/Fall2023_IDS706_MiniProject8_JiayiZhou/actions/workflows/rustCI.yml/badge.svg)](https://github.com/nogibjj/Fall2023_IDS706_MiniProject8_JiayiZhou/actions/workflows/rustCI.yml)

### Purpose
This is for class data engineering mini project 8. It rewrites a Python Script in Rust. It takes an existing Python script for data processing, rewrites it in Rust, and highlights improvements in speed and resource usage.

### Requirements
  * Take an existing Python script for data processing
  * Rewrite it in Rust
  * Highlight improvements in speed and resource usage

### Functionality
The code (both in Python and Rust) does: ETL-Query: [E] Extract a dataset from URL, [T] Transform, [L] Load into SQLite Database and [Q] Query For the ETL-Query lab:
  * [E] Extract a dataset from a URL with CSV format.
  * [T] Transform the data by cleaning, filtering, enriching, etc to get it ready for analysis.
  * [L] Load the transformed data into a SQLite database table using Python's sqlite3 module.
  * [Q] Accept and execute general SQL queries on the SQLite database to analyze and retrieve insights from the data.

### Steps
I cloned the project template created by Professor Noah Gift and modified the template. Based on the template from professor, I made the following changes:
1. Cargo init to add rust constructions
2. Update makefile
3. Create yml file separately for python and rust
4. Modify python functions libraries based on mini project 5
5. Create and modify cargo files to match with the functions in python
6. Add log function that will keep track of the time and computer memoery uesed

### Rust Implementation
It measures and logs the elapsed time for the extraction operation specifically and appends this information to a Markdown file named `rust_query_log.md`.

##### Preparation and Dependency Installation:
1. open codespaces 
2. wait for codespaces to be built 
3. build: `cargo build` for dependencies installation
4. run: `cargo run`

##### Check Format and Test Erros: 
1. Format code `make format`
2. Lint code `make lint`
3. Test coce `make test`

### Python Implementation
It measures and logs the elapsed time for the extraction operation specifically and appends this information to a Markdown file named `python_query_log.md`.

##### Preparation and Dependency Installation:
1. open codespaces 
2. wait for codespaces to be built
3. install: `make python_install`

### Check Format and Test Errors: 
1. Format code `make python_format`
2. Lint code `make python_lint`
3. Test coce `make python_test`

### Comparison
The python time and memory usage file is [here](https://github.com/nogibjj/Fall2023_IDS706_MiniProject8_JiayiZhou/blob/main/python_query_log.md) and the Rust time and memory usage file is [here](https://github.com/nogibjj/Fall2023_IDS706_MiniProject8_JiayiZhou/blob/main/rust_query_log.md). Also, as we can see from the screenshots below, python take more memory and more time to process the same function.
<img width="655" alt="Screenshot 2023-10-21 at 9 52 05 PM" src="https://github.com/nogibjj/Fall2023_IDS706_MiniProject8_JiayiZhou/assets/143651921/2c0de291-e319-4828-967a-6db71ce468ac">
<img width="625" alt="Screenshot 2023-10-21 at 9 51 59 PM" src="https://github.com/nogibjj/Fall2023_IDS706_MiniProject8_JiayiZhou/assets/143651921/26b489c1-5cc5-4347-ab6e-41c592398d28">





