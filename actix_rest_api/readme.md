## Actix REST API

This is an example of a RESTful API implementation, exposing a CRUD to an entity (user), using this stack:
- Actix Web - as the Web framework
- Diesel - as the ORM
- PostgreSQL - as the persistence store

The code is mainly based on this nice [blog entry](https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/), 
I just fixed the update operation and did some small updates (as per current support of `uuid` in `diesel`) to the dependencies.


### Setup

- for auto reload, do the following:
  - install some utilities: `cargo install systemfd cargo-watch`
  - run using: `systemfd --no-pid -s http::3000 -- cargo watch -x run`<br/>
    (although it behaves the same as using `cargo watch -x run`)    
- start/expose a PostgreSQL DB access
  - you can use a Docker image<br/>
    `docker-compose up -d`
  - add the access variable (DATABASE_URL) to `.env` file<br/>
    `DATABASE_URL="postgres://postgres:common@localhost/actix_rest_api`
- install Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`


### Usage

The implemented use cases (operations) are listed below with cURL examples.

- get all users:
  ```bash
  curl -v localhost:3000/users
  ```
- get a user:
  ```bash
  curl -v localhost:3000/users/{id-of-the-existing-user}
  ```
- create a user:
  ```bash
  curl -v -X POST -d '{ "email": "joe@mail.com", "password": "joe" }' localhost:3000/users -H "content-type: application/json"
  ```
- update a user:
  ```bash
  curl -v -X PUT -d '{ "email": "joe@mail.com", "password": "joe1" }' localhost:3000/users/{id-of-the-existing-user} -H "content-type: application/json"
  ```
- delete a user:
  ```bash
  curl -v -X DELETE localhost:3000/users/{id-of-the-existing-user}
  ```
