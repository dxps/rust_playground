# A todos CRUD Restful API using Warp

## Packages

| package | description |
| --- | --- | 
| `warp` | the Web framework |
| `tokio` | the async runtime, needed to execute futures |
| `mobc` / `mobc-postgres` | the asynchronous database connection pool |
| `serde` | is used for serializing and deserializing objects to/from JSON |
| `chrono` | brings in date and time utilities |
| `thiserror` | is an utility library used for error handling |

<br/>

## Start

### The Database

Use `run_db.sh` script to start a local Docker container of PostgreSQL 12.

### The Server

The standard `cargo run` starts the Warp server, does the startup initialization, and mounts the routes. The server listens on `localhost:8000`.

<br/>

## Usage

Check health using `curl -v http://localhost:8000/health`

<br/>
