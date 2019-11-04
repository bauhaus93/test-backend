FROM debian:sid

RUN useradd server -p secret
RUN mkdir -p /app
WORKDIR /app

RUN apt-get update && \
	apt-get -y install curl

USER server
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh && \
	chmod +x rustup.sh && \
	./rustup.sh -y --default-toolchain=stable
ENV PATH="$PATH:$HOME/.cargo/bin"
COPY ./ ./
RUN cargo run --release && \
	cp target/release/main ./server_app && \
	chown server:server ./server_app

EXPOSE 10001
ENTRYPOINT /app/server_app
