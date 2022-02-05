FROM rust:1.58

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build --release --verbose

WORKDIR /app/target/release

ARG port=5000
EXPOSE $port

# THIS DEFAULT VALUE SHOULD NOT BE USED, The user should set it
ENV ROCKET_SECRET_KEY="KN9CHTfQpGWwgM6w3fm+yqmhZxX0RhjA5c+kH5xiHZ0="
ENV ROCKET_PORT=$port
ENV ROCKET_ADDRESS=0.0.0.0

CMD ["./prexel-server"]
