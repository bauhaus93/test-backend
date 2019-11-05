FROM rust
WORKDIR /app/src
COPY . .
RUN RUST_LOG="test_backend=debug,main=debug" cargo install --path .
CMD ["server_app"]
