# Simple Rust-based HTTP Server

This project showcases a couple of Rust's features.

It uses just the Rust's standard features, no external dependencies (crates) are used in this simplistic example, used for demonstration and learning purposes.

## Run

Having the source code, you can use the standard `cargo run`.

The location from where to serve files can be provided as a value to `PUBLIC_PATH` environment variable, the default value being the `public` directory from the current path.

So, you can use `PUBLIC_PATH=/some/path cargo run` or set it before running.

## Directory Traversal Protection

This feature is included, and since nowadays the browsers dont' allow you to put `..` in the URL, an alternative way to test it is to use:
```shell script
$ echo "GET ../readme.md HTTP/1.1\r\n\r\n" | netcat localhost 8080
```
