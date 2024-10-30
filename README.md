[![Rust CI/CD Pipeline](https://github.com/nogibjj/Kaisen_Yao_IDS706_Week8_Individual/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/Kaisen_Yao_IDS706_Week8_Individual/actions/workflows/ci.yml)

# Kaisen Yao IDS706 Week 8 Individual
## ETL and Database Query Tool in Rust

This project presents a streamlined tool in Rust for data Extraction, Transformation, Loading (ETL), and querying using a SQLite database. The project originates from earlier Python implementations, now enhanced in Rust with error handling and improved reliability.

## Project Walkthrough Video 
[Watch the Project on YouTube](https://youtu.be/)

## Process Overview
![ETL Process Flow](etl_flow.svg)

## Key Features

1. **Data Extraction**
   - `extract`: Downloads data from a specified URL and stores it locally for processing.

2. **Data Transformation & Loading**
   - `transform_load`: Processes a CSV dataset, structures the records, and loads them into a SQLite table named `US_births`.

3. **Flexible Database Querying**
   - `query`: Executes SQL commands (SELECT, INSERT, UPDATE, DELETE) and logs them into `query_log.md` for tracking and auditing.

4. **Automated Logging**
   - `log_query`: Records each SQL query into a markdown log file, keeping an organized history of operations.

## Setup & Execution
1. Open Codespaces and wait for initialization.
2. Run `cargo build` to install dependencies.
3. Extract data: `cargo run extract`.
4. Transform and load data: `cargo run transform_load`.
5. Use CRUD samples with `make create`, `make read`, `make update`, `make delete`.
6. For custom queries: `cargo run query "<Your SQL query here>"`.
7. Access the query log for successful operations [here](https://github.com/nogibjj/Kaisen_Yao_IDS706_Week8_Individual/blob/main/query_log.md).

## Code Maintenance and Testing
1. Format code with `make format`.
2. Lint code using `make lint`.
3. Run tests to verify functionality with `make test`.

## Access Compiled Rust Binary
- To download the built binary, navigate to `Actions` and open the latest workflow run.

## Use of LLM: Leveraging Language Models in Development
Throughout the development of this project, a Language Learning Model (ChatGPT 4o) was utilized to enhance productivity and ensure accuracy. The LLM assisted in:

- Translating and adapting Python code into Rust, particularly for data manipulation and ETL operations.
- Troubleshooting issues in Rust, such as handling borrow checker rules and optimizing error handling.
- Crafting detailed documentation, including clarifications in `README.md`, test explanations, and function descriptions to streamline the project setup and execution.

The use of an LLM provided valuable insights and expedited the overall development process by offering guidance on Rust-specific conventions and best practices.

## Reference
* [Rust CLI Template Guide](https://github.com/kbknapp/rust-cli-template)
* [Rust Data Engineering Resources](https://github.com/nogibjj/rust-data-engineering)
* [SQLite for Rust Documentation](https://docs.rs/sqlite/latest/sqlite/)
* [Data Source: FiveThirtyEight](https://github.com/fivethirtyeight/data)