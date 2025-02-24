FROM core-builder as builder

ADD . .

RUN cargo +nightly build -p deposit-vault --release \
        --out-dir bin -Z unstable-options && \
    cargo clean


# build deposit-vault
FROM debian:buster-slim

RUN apt update
RUN apt install -y libssl-dev libpq-dev

COPY --from=builder /ezex/bin/deposit-vault /usr/bin

CMD ["deposit-vault", "start"]
