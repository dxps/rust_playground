#!/bin/sh

docker run --name warp_todo -e POSTGRES_PASSWORD=postgres -p 7999:5432 -d postgres:12
