```markdown
# Blazingly Fast REST API in Rust

![Rust Version](https://img.shields.io/badge/rust-1.78.0-orange.svg)
![Framework](https://img.shields.io/badge/framework-Axum-blue.svg)
![Database](https://img.shields.io/badge/database-SQLx_/_SQLite-green.svg)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)

This project is a high-performance, asynchronous REST API built in Rust using Axum and SQLx. It delivers a persistent To-Do list service and is optimized for clarity, scalability, and performance. 

The primary goal was to benchmark Rust against FastAPI under high-concurrency conditions and showcase the real-world benefits of Rust's async runtime, compile-time safety, and memory efficiency.

---

## Project Structure

```

.
├── Cargo.toml
├── .env              # SQLite connection string
├── migrations/       # Database schema migrations
│   └── ...
└── src/
├── main.rs       # Application entry point
├── routes.rs     # Route definitions
├── handlers.rs   # Endpoint logic
├── db.rs         # Database interactions
├── models.rs     # Shared structs and models
└── errors.rs     # Custom error handling

````

---

## Features

- Fully asynchronous using `tokio` and `axum`
- Compile-time SQL validation with `sqlx`
- Persistent SQLite storage
- Clean, modular architecture
- Custom error handling
- High throughput and minimal latency under load

---

## Getting Started

### Prerequisites

- Rust toolchain (install via [rustup.rs](https://rustup.rs))
- `sqlx-cli` for migrations:  
  ```bash
  cargo install sqlx-cli
````

### Installation

1. Clone the repository:

   ```bash
   git clone git@github.com/ishworii/todo_api_rust 
   cd todo_api_rust 
   ```

2. Create a `.env` file:

   ```
   DATABASE_URL=sqlite:todos.db
   ```

3. Initialize the database:

   ```bash
   sqlx database create
   sqlx migrate run
   ```

4. Run the server (use release mode for benchmarking):

   ```bash
   cargo run --release
   ```

   The server will be available at `http://127.0.0.1:3000`.

---

## Benchmark Results

Benchmarks were conducted using [`oha`](https://github.com/hatoo/oha) with 50 concurrent clients for 15 seconds. Rust's Axum API was compared with FastAPI (Uvicorn, 4 workers).

### POST `/todos` (Write-heavy benchmark)

| Metric              | FastAPI (4 workers) | Rust (Axum) | Gain         |
| ------------------- | ------------------- | ----------- | ------------ |
| Requests/sec        | 1,263               | 15,760      | 12.5× faster |
| Avg Latency         | 38.5 ms             | 3.2 ms      | 12× lower    |
| 99.9% Latency       | 2.09 s              | 0.47 s      | 4.4× lower   |
| Successful Requests | 18,902              | 236,466     | 12.5× more   |

**Conclusion**: The Rust implementation handled 12 times more requests with significantly lower latency, thanks to its compiled performance model, non-blocking async runtime, and efficient resource management.

---

### GET `/todos/5` (Read-heavy, single-record benchmark)

| Metric        | FastAPI | Rust    |
| ------------- | ------- | ------- |
| Requests/sec  | 7,862   | 76,375  |
| Avg Latency   | 6.4 ms  | 0.7 ms  |
| 99.9% Latency | 119 ms  | 12.5 ms |

**Conclusion**: Rust handled 10 times more requests per second with lower and more predictable latency, even in read scenarios.

---

### GET `/todos` (Fetch all records, \~236k entries)

| Metric        | Result                |
| ------------- | --------------------- |
| Requests/sec  | 4.3                   |
| Avg Latency   | 7.9 s                 |
| Response Size | \~33.2 MB per request |

**Conclusion**: This test stressed the server with large payloads. The Rust server remained stable, consistently serving 33 MB JSON responses without crashes or degradation, proving its reliability under extreme loads.

---

## Lessons Learned

* Rust's async stack is ready for production and performs exceptionally well under high concurrency.
* The performance gap between Python and Rust is substantial, especially in write-heavy or CPU-bound tasks.
* SQLite with `sqlx` provides a safe and reliable storage layer, with strong compile-time guarantees.
* Memory safety and absence of garbage collection give Rust more predictable latency under load.

---

## Conclusion

This project demonstrates that Rust is a compelling option for building backend APIs where performance and reliability are critical. While Python frameworks like FastAPI offer excellent developer experience, Rust provides a clear edge in raw throughput, latency, and stability.

---

## Future Enhancements

* Replace SQLite with PostgreSQL for better concurrency and scaling
* Add full CRUD operations, pagination, and filtering
* Integrate authentication using JWT
* Add a WebSocket endpoint for real-time updates
* Deploy using Docker and set up CI/CD pipelines

---

## License

Licensed under either of:

* MIT License
* Apache License, Version 2.0


