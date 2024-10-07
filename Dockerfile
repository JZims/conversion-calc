FROM rust:1.76.0
WORKDIR /usr/src/concal
COPY . .
RUN cargo install --path .
