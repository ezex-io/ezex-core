FROM core-builder as builder

ADD . .

RUN cargo build --profile ezex-deposit --release

FROM debian:buster-slim

RUN apt update
RUN apt install -y libssl-dev libpq-dev

COPY --from=builder /ezex/target/release/ezex-deposit /usr/bin

CMD ["ezex-deposit", "start"]
