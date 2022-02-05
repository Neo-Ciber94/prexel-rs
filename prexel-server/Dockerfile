FROM rust:1.58

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build --release --verbose

WORKDIR /app/target/release

ARG port=5000
ENV ROCKET_PORT=$port
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE $port

CMD ["./prexel-server"]
