#!/bin/sh

## `COMPOSE_PROJECT_NAME` is relevant here as it helps preventing those warnings and conflicts:
## "WARNING: Found orphan containers (...) for this project."

COMPOSE_PROJECT_NAME=warp_todo_crud_ex2_db docker-compose -f ./ops/docker-compose.yml up -d
