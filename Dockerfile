FROM rust
WORKDIR /app/src
COPY . .
RUN cargo install --path .
ENV RUST_LOG "test_backend=debug,server_app=debug" 
CMD ["server_app"]
