# Axum based URL Shortener

An URL shortener sample app built with [axum](https://github.com/tokio-rs/axum) and [tokio](https://github.com/tokio-rs/tokio).

<br/>

## Run

Use the classic `cargo run` command to run the server.

<br/>

## Usage

-   Shorten an URL using `curl -v -X POST localhost:3000/shorten -d http://some.com`
    ```shell
    ❯ curl -X POST localhost:3000/shorten -d http://some.com
    {"shortened_url":"u_FVM4ZgtHZDUG81"}
    ❯
    ```
-   Get the original URL using `curl -v localhost:3000/u_FVM4ZgtHZDUG81`
    ```shell
    ❯ curl localhost:3000/unshorten/u_FVM4ZgtHZDUG81
    {"long_url":"http://some.com"}
    ❯
    ```
