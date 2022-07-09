#!/bin/sh

## Using benchrs to stress it.
## If you don't have it already, use `cargo install benchrs` to have it.

benchrs -c 50 -n 10000 -k http://127.0.0.1:8000

