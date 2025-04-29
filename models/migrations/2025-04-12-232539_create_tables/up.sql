-- Create accounts table
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stellar_address TEXT NOT NULL,
    email TEXT,
    name TEXT,
    phone TEXT,
    status TEXT NOT NULL,
    kyc_status TEXT NOT NULL,
    memo TEXT,
    memo_type TEXT,
    balance NUMERIC(20, 8),
    last_login TIMESTAMP,
    last_kyc_submitted TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create offramp_transactions table
CREATE TABLE offramp_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    transaction_id TEXT NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    dest_currency TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create offramp_quotes table
CREATE TABLE offramp_quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID NOT NULL REFERENCES offramp_transactions(id) ON DELETE CASCADE,
    quote_id TEXT NOT NULL,
    sell_asset TEXT NOT NULL,
    buy_asset TEXT NOT NULL,
    sell_amount NUMERIC(20, 8) NOT NULL,
    buy_amount NUMERIC(20, 8) NOT NULL,
    price NUMERIC(20, 8) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- migrations/YYYY-MM-DD-HHMMSS_create_sep38_assets/up.sql
CREATE TABLE sep38_assets (
    id SERIAL PRIMARY KEY,
    asset VARCHAR NOT NULL UNIQUE,
    sell_delivery_methods JSONB,
    buy_delivery_methods JSONB,
    country_codes JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- migrations/YYYY-MM-DD-HHMMSS_create_sep38_quotes/up.sql
CREATE TABLE sep38_quotes (
    id UUID PRIMARY KEY,
    original_quote_id VARCHAR NOT NULL,
    sell_asset VARCHAR NOT NULL,
    buy_asset VARCHAR NOT NULL,
    sell_amount NUMERIC NOT NULL,
    buy_amount NUMERIC NOT NULL,
    price NUMERIC NOT NULL,
    total_price NUMERIC NOT NULL,
    fee_total NUMERIC NOT NULL,
    fee_asset VARCHAR NOT NULL,
    fee_details JSONB,
    sell_delivery_method VARCHAR,
    buy_delivery_method VARCHAR,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    context VARCHAR NOT NULL,
    transaction_id UUID
);

CREATE INDEX idx_sep38_quotes_original_quote_id ON sep38_quotes(original_quote_id);
CREATE INDEX idx_sep38_quotes_transaction_id ON sep38_quotes(transaction_id);
-- Index for fast lookup of quotes by transaction
CREATE INDEX idx_offramp_quotes_transaction_id ON offramp_quotes(transaction_id);
