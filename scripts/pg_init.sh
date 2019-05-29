#!/bin/sh

sudo mkdir -p /run/postgresql
sudo chown jakob /run/postgresql

rm -rf pg_cluster/
mkdir pg_cluster/

echo "Creating DB cluster"
initdb -D pg_cluster -D pg_cluster/

scripts/pg_start.sh

echo "Creating DB 'backend_prod'"
createdb backend_prod
psql -e -d backend_prod -f scripts/sql/setup.sql

echo "Creating DB 'backend_test'"
createdb backend_test
psql -e -d backend_test -f scripts/sql/setup.sql

scripts/pg_stop.sh
