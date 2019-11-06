#!/bin/sh

RUST_LOG="test_backend=debug,server_app=debug" cargo run --release
