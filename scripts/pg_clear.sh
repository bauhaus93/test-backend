#!/bin/sh
psql -d backend_test -a -f scripts/sql/clear.sql
psql -d backend_test -a -f scripts/sql/setup.sql

psql -d backend_prod -a -f scripts/sql/clear.sql
psql -d backend_prod -a -f scripts/sql/setup.sql