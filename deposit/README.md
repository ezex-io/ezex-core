# ezeX Deposit service

ezeX Deposit is a service to create and manage users' deposit addresses.
Generated addresses are assign to the user for the depositing cryptocurrencies.
User can only have one deposit address per Blockchain protocol.

## Security


## How to run

To run the service execute this command:

```
deposit start -h
```

This commands print the list of arguments to start the service.
The command line arguments can be set through the environment variables.


```sh
    --grpc-address <address>                          [env: GRPC_ADDRESS=]
    --redis-connection-string <connection-string>     [env: REDIS_CONNECTION_STRING=]
    --redis-consumer <consumer-name>                  [env: REDIS_CONSUMER=]
    --database-url <db-url>                           [env: DATABASE_URL=]
    --default-wallet <default-wallet>                 [env: DEFAULT_WALLET=]  [default: spot]
    --log-file <file>                                 [env: LOG_FILE=]  [default: ]
    --redis-group-name <group-name>                   [env: REDIS_GROUP_NAME=]
    --log-level <level>                               [env: LOG_LEVEL=]  [default: trace]
    --pool-size <pool-size>                           [env: DATABASE_POOL_SIZE=]  [default: 3]
```
