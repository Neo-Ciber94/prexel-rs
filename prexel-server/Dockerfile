FROM rust:1.58 as build

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build --release --verbose

WORKDIR /app/target/release

ARG port=5000
ENV ROCKET_PORT=$port

CMD ["./prexel-server"]
