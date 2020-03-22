### Setup

-   for auto reload, do the following:
    -   install some utilities: `cargo install systemfd cargo-watch`
    -   run using: `systemfd --no-pid -s http::3000 -- cargo watch -x run`
-   start/expose a PostgreSQL DB access
    -   you can use a Docker image<br/>
        `...`
    -   add the access variable (DATABASE_URL) to `.env` file<br/>
        `DATABASE_URL="postgres://postgres:common@localhost/actix_rest_api`
-   install Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

### Usage

The following use cases (operations) are implemented:

-   get all users:
    ```bash
    curl -s localhost:3000/users | jq
    ```
-   create a user:
    ```bash
    curl -v -X POST -d '{ "id": 3, "email": "a@b.com" }' localhost:3000/users -H "content-type: application/json"
    ```
-   update a user:
    ```bash
    curl -v -X PUT -d '{ "id": 3, "email": "a@c.com" }' localhost:3000/users -H "content-type: application/json"
    ```
