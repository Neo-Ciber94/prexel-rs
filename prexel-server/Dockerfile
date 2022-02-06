FROM rust:1.58

WORKDIR /app
COPY . .

RUN cargo build --release --verbose
WORKDIR /app/target/release

ENV PORT=5000

CMD ["./prexel-server"]
