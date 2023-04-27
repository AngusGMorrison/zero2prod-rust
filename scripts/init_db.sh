#!/usr/bin/env bash

set -eo pipefail

if ! command -v pg_isready &> /dev/null; then
  echo "Error: pg_isready is not installed." >&2
  exit 1
fi

if ! command -v sqlx &> /dev/null; then
  echo "Error: sqlx is not installed." >&2
  exit 1
fi

DB_USER=$ZERO2PROD_POSTGRES_USER
DB_PASSWORD=$ZERO2PROD_POSTGRES_PASSWORD
DB_NAME=$ZERO2PROD_POSTGRES_DB
DB_PORT=$ZERO2PROD_POSTGRES_PORT
DB_HOST=$ZERO2PROD_POSTGRES_HOST
DB_URL=$ZERO2PROD_POSTGRES_URL

# Launch postgres using Docker
echo "Launching postgres..."
docker run \
  --env POSTGRES_USER=$DB_USER \
  --env POSTGRES_PASSWORD=$DB_PASSWORD \
  --env POSTGRES_DB=$DB_NAME \
  --publish $DB_PORT:5432 \
  --detach \
  postgres \
  postgres -N 1000 > /dev/null # increase max connections for testing purposes

# Await Postgres.
SLEEP_SECONDS=1
until pg_isready -d $DB_NAME -h $DB_HOST -p $DB_PORT -U $DB_USER --quiet; do
  echo "Postgres unavailable. Retrying in $SLEEP_SECONDS s."
  sleep $SLEEP_SECONDS
done
echo "Postgres available on port $DB_PORT."

echo "Creating database..."
sqlx database create --database-url=$DB_URL
echo "Database created."