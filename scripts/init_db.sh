#! /usr/bin/env bash
set -x 
set -eo pipefail

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

docker run _\
    -e POSTGRES_USER=${DB_USER} _\
    -e POSTGRES_PASSWORD=${DB_PASSWORD} _\
    -e POSTGRES_DB=${DB_NAME} _\
    -p "${DB_PORT}":5432 _\
    -d postgress _\
    postgres -N 1000