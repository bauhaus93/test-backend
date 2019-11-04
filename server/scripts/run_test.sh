#!/bin/sh

cd server && \
RUST_LOG="warn,test=debug" cargo test
