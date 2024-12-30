#!/usr/bin/env bash

set -x
set -eo pipefail

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWD="${POSTGRES_PASSWORD:=password}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_NAME="${POSTGRES_DB:=newsletter}"

export PGPASSWORD="${DB_PASSWD}"

function check_deps {
  local fail_flag=false
  for cmd; do
    if ! [ -x "$(command -v "$cmd")" ]; then
      echo >&2 "\e[31;1mMissing dependency tool: '$cmd'\e[0m"
      fail_flag=true
    fi
  done
  if $fail_flag; then
    exit 1
  fi
}

check_deps psql sqlx

if ! psql 2>/dev/null -h localhost -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c "\q"; then
  echo -e >&2 "\e[33;1mNo running instance of postgres found.\e[0m"
  docker rm 2>/dev/null -f newsletter_db
  echo -e >&2 "\e[32;1mCreating new instance of postgres...\e[0m"
  docker run \
    --env "POSTGRES_USER=${DB_USER}" \
    --env "POSTGRES_PASSWORD=${DB_PASSWD}" \
    --env "POSTGRES_DB=${DB_NAME}" \
    --publish "${DB_PORT}:5432" \
    --name "newsletter_db" \
    --detach postgres \
    postgres -N 1000
fi

until psql 2>/dev/null -h localhost -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c "\q"; do
  echo -e >&2 "\e[31;1mPostgres still inactive....\e[0m"
  sleep 2
done

echo -e >&2 "\e[32;1mPostgres is active.\e[0m"

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWD}@localhost:${DB_PORT}/${DB_NAME}"

sqlx database create
# sqlx migrate add create_subscriptions_table
sqlx migrate run

echo -e >&2 "\e[32;1mMigration successful.\e[0m"

