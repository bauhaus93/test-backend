#!/bin/sh

RUST_LOG="test_backend=debug,main=debug" cargo run --release
