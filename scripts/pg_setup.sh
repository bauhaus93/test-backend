#!/bin/sh

dropdb --if-exists backend_prod
dropdb --if-exists backend_test

echo "Creating DB 'backend_prod'"
createdb backend_prod
psql -e -d backend_prod -f scripts/sql/setup.sql

echo "Creating DB 'backend_test'"
createdb backend_test
psql -e -d backend_test -f scripts/sql/setup.sql
