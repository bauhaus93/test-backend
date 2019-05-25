#!/bin/sh

sudo mkdir -p /run/postgresql
sudo chown jakob /run/postgresql

rm -rf pg_cluster/
mkdir pg_cluster/

echo "Creating DB cluster"
initdb -D pg_cluster -D pg_cluster/ -E UTF-8 --locale=de_AT.utf8

scripts/pg_start.sh

echo "Creating DB 'backend_production'"
createdb backend_production
echo "Creating DB 'backend_test'"
createdb backend_test

scripts/pg_stop.sh