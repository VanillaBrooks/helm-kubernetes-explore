FROM docker.io/rust:latest

RUN apt update -y && apt upgrade -y

RUN mkdir /static-service

WORKDIR /static-service
RUN cargo init .
COPY Cargo.lock ./
COPY Cargo.toml ./

RUN cargo build

COPY src/ ./src/

RUN cat Cargo.toml
RUN ls ./
RUN ls ./src
# incremental compilation and linking of new source code
RUN cargo install --path . --debug

ENTRYPOINT static-service
