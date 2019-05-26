#!/bin/sh

RUST_LOG="test-backend=debug,main=debug" cargo run --release
