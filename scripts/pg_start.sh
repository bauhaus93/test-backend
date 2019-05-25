#!/bin/sh

rm postgres.log
pg_ctl -D pg_cluster/ -l postgres.log start