FROM debian:sid

RUN useradd server -p secret -d /app/
WORKDIR /app

RUN apt-get update && \
	apt-get -y install curl

ENV PATH="$PATH:$HOME/.cargo/bin"

RUN chown -R server /app/

USER server
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /app/rustup.sh && \
	chmod +x /app/rustup.sh && \
	/app/rustup.sh -y --default-toolchain=stable
COPY ./ ./
RUN cargo run --release && \
	cp target/release/main /app/server_app && \
	chown server:server /app/server_app

EXPOSE 10001
ENTRYPOINT /app/server_app
