## About

This folder contains the database migration scripts.

<br/>

## Usage

Such scripts are being created and applied using `sqlx` tool.
- To install it use `cargo install sqlx-cli`.
- To create a new script use `sqlx migrate add {name}` (for ex, `{name}` can be `api_key`).
- To apply it use `sqlx migrate run`

