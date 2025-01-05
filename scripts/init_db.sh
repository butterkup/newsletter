#!/usr/bin/env bash

# set -x
set -eo pipefail

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWD="${POSTGRES_PASSWORD:=password}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_NAME="${POSTGRES_DB:=newsletter}"

export PGPASSWORD="${DB_PASSWD}"

ERROR_COLOR="\e[31;1m"
OKAY_COLOR="\e[32;1m"
INFO_COLOR="\e[34;1m"

function _logger {
  echo -e >"${LOG_FILE:-/dev/stderr}" "$color$@\e[0m"
}

function log_info {
  color="$INFO_COLOR" _logger "$@"
}

function log_error {
  color="$ERROR_COLOR" _logger "$@"
}

function log_okay {
  color="$OKAY_COLOR" _logger "$@"
}

function check_deps {
  local fail_flag=false
  for cmd; do
    if ! [ -x "$(command -v "$cmd")" ]; then
      log_error "Missing dependency tool: '$cmd'"
      fail_flag=true
    fi
  done
  if $fail_flag; then
    exit 1
  fi
}

log_info "Looking for dependencies"
check_deps psql

if ! psql 2>/dev/null -h localhost -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c "\q"; then
  log_error "No running instance of postgres found"
  docker rm 2>/dev/null -f newsletter_db
  log_info "Creating new instance of postgres"
  if docker run \
    --env "POSTGRES_USER=${DB_USER}" \
    --env "POSTGRES_PASSWORD=${DB_PASSWD}" \
    --env "POSTGRES_DB=${DB_NAME}" \
    --publish "${DB_PORT}:5432" \
    --name "newsletter_db" \
    --detach postgres \
    postgres -N 1000; then
    log_okay "Postgres instance created"
  else
    log_error "Postgres instance creation failed"
  fi
else
  log_okay "Found postgres instance"
fi

until psql 2>/dev/null -h localhost -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c "\q"; do
  log_error "Postgres still inactive. Going to sleep"
  sleep 2
done

log_okay "Postgres is active"

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWD}@localhost:${DB_PORT}/${DB_NAME}"
echo "DATABASE_URL='$DATABASE_URL'" >"${ENV_FILE:=.env}"

log_info "Creating database and running migrations"

cargo sqlx database create
# sqlx migrate add create_subscriptions_table
cargo sqlx migrate run

log_okay "Migrated successfully"

