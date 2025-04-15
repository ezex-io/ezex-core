CREATE TABLE addresses (
    user_id VARCHAR NOT NULL,
    chain_id VARCHAR NOT NULL,
    wallet_id VARCHAR NOT NULL,
    deposit_address VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, chain_id)
);
