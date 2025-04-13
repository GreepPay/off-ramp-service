-- migrations/YYYY-MM-DD-HHMMSS_create_offramp_tables/up.sql
CREATE TABLE offramp_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    transaction_id TEXT NOT NULL UNIQUE,
    asset_code TEXT NOT NULL DEFAULT 'USDC',
    amount NUMERIC(20, 7) NOT NULL,
    dest_currency TEXT NOT NULL,
    status TEXT NOT NULL,
    memo TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_offramp_transactions_account ON offramp_transactions(account_id);
CREATE INDEX idx_offramp_transactions_status ON offramp_transactions(status);

CREATE TABLE offramp_quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID NOT NULL REFERENCES offramp_transactions(id),
    quote_id TEXT NOT NULL UNIQUE,
    sell_asset TEXT NOT NULL,
    buy_asset TEXT NOT NULL,
    sell_amount NUMERIC(20, 7) NOT NULL,
    buy_amount NUMERIC(20, 7) NOT NULL,
    price NUMERIC(20, 7) NOT NULL,
    fee NUMERIC(20, 7),
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_offramp_quotes_transaction ON offramp_quotes(transaction_id);