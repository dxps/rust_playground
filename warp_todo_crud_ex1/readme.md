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

### Check health

Using `curl -v http://localhost:8000/health`

### Get all todos

Using `curl -X GET 'http://localhost:8000/todo/' -H 'Content-Type: application/json'`

### Search todos by name

Using `curl -X GET 'http://localhost:8000/todo/?search=Done%20Todo' -H 'Content-Type: application/json`

### Add todo

Using `curl -X POST 'http://localhost:8000/todo/' -H 'Content-Type: application/json' -d '{"name": "Done Todo"}'`

### Update todo

Using `curl -X PUT 'http://localhost:8000/todo/2' -H 'Content-Type: application/json' -d '{"name": "Done Todo", "checked": true}'`

### Delete todo

Using `curl -v -X DELETE 'http://localhost:8000/todo/2' -H 'Content-Type: application/json'`

<br/>
