#!/bin/sh

rm postgres.log
pg_ctl -D pg_cluster/ -l postgres.log start

psql -d backend_test -a -f scripts/sql/clear.sql
psql -d backend_test -a -f scripts/sql/setup.sql