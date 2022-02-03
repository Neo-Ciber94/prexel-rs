FROM rust:1.58-alpine

WORKDIR /app
COPY ./target/release/ .

ARG PORT 
ENV ROCKET_PORT=$PORT

CMD ["prexel-server"]