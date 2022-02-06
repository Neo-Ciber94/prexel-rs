FROM rust:1.58

WORKDIR /app
COPY . .

RUN cargo build --release --verbose
WORKDIR /app/target/release

ARG port=5000
EXPOSE $port
ENV ACTIX_PORT=$port

CMD ["./prexel-server"]
