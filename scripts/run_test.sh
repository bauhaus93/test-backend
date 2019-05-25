#!/bin/sh

scripts/pg_start.sh
RUST_LOG="warn,test=debug" cargo test
scripts/pg_stop.sh