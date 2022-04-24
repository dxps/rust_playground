# A Todo Web Sample using Actix

This is a sample web app made using Actix.

## Setup

1. Start the database
   For your convenience, a Docker image is provided.<br/>
   Just run `docker-compose up -d` in a terminal to start the PostgreSQL database.
2. Create the physical data model
   Use `database.sql` file to create the tables (using either a `psql` client or any other IDE).

## Run

Use the standard `cargo run` to run it.

## Usage

Access / using:
```bash
$ curl -s localhost:8080 | jq
{
  "status": "UP"
}
$
```

Get all todo lists:
```bash
$ curl -s localhost:8080/todos | jq
[
  {
    "id": 1,
    "title": "List 1"
  },
  {
    "id": 2,
    "title": "List 2"
  }
]
$
```

Get the items of a todo list:
```shell script
$ curl -s localhost:8080/todos/1/items | jq
[
  {
    "id": 1,
    "title": "Item 1",
    "checked": false,
    "list_id": 1
  },
  {
    "id": 3,
    "title": "Item 2",
    "checked": false,
    "list_id": 1
  }
]
$ 
```

Create a new todo list:
```shell script
$ curl -X POST -H "content-type: application/json" -d '{"title":"List 3"}' localhost:8080/todos
  {"id":3,"title":"List 3"}%
$
```
