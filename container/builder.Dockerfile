FROM rustlang/rust:nightly-buster-slim as core-builder

RUN apt update
RUN apt install -y protobuf-compiler libssl-dev pkg-config libpq-dev



