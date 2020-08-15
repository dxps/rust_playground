#!/bin/sh

## Notes: The `--rm` flag tells Docker to remove the container 
##        (and any associated file system) when container exits.

docker run --name warp_todo --rm -e POSTGRES_PASSWORD=postgres -p 7999:5432 -d postgres:12

