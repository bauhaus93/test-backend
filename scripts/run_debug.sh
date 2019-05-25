#!/bin/sh

scripts/pg_start.sh
RUST_LOG="test-backend=debug,main=debug" cargo run
scripts/pg_stop.sh