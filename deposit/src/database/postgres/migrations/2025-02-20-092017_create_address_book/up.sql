-- Enable uuid extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS deposit_wallets (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  active BOOLEAN NOT NULL,
  wallet_id VARCHAR NOT NULL,
  chain_id VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS deposit_address_book (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  status SMALLINT NOT NULL,
  user_id VARCHAR NOT NULL,
  chain_id VARCHAR NOT NULL,
  wallet_id UUID NOT NULL REFERENCES deposit_wallets (id),
  address VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
