# ezeX-Core Services

ezeX-Core contains core services of ezeX exchange.

| Service name                         |
| ------------------------------------ |
| [Deposit](./deposit/vault/README.md) |

## Getting Started

We are using PostgresQL for our database. Make sure you have PostgreSQL driver before compiling the project.
In linux machine you can run this command to install PostgreSQL driver:

```bash
apt install libpq-dev    # Postgress driver
apt install libssl-dev   # OpenSSL
apt install redis        # Redis
```

Make sure you have the latest stable version of [Rust](https://www.rust-lang.org/tools/install) installed before continuing.
Once Rust has been installed, it's dependency manager Cargo can be used to install additional tooling for local development.

```
cargo build
```

## Testing

By running`cargo test` you can test all the libraries and binaries.
If you like to perform only unit testing, run `cargo test --lib`.

## Contributing

Please read [Contribution](./CONTRIBUTING.md) document before start working on this project.

## Dockers

Please read [Docker containers](./container/README.md) document to create docker images.

## Deployment

Please read [Deployment](./DEPLOYMENT.md) document before publishing/deploying the services.

## License

This software is under commercial License
