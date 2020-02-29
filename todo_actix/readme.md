# A Todo Web Sample using Actix

This is a sample web app made using Actix.

## Setup

1. Start the database
   For your convenience, a Docker image is provided.<br/>
   Just run `docker-compose up -d` in a terminal to start the PostgreSQL database.
2. Create the physical data model
   Use `database.sql` file to create the tables (using either a `psql` client or any other IDE).

## Run

Use the classic `cargo run` to start the project.

## TODOs

- check database connection health at startup
