CREATE TABLE offramp_service.sep12_customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    account TEXT NOT NULL,
    memo TEXT,
    memo_type TEXT,
    customer_type TEXT NOT NULL,
    status TEXT NOT NULL,
    first_name TEXT,
    last_name TEXT,
    email TEXT,
    phone TEXT,
    date_of_birth TEXT,
    address_street TEXT,
    address_city TEXT,
    address_state TEXT,
    address_postal_code TEXT,
    address_country TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
);

CREATE INDEX idx_sep12_customers_account ON offramp_service.sep12_customers (account);

CREATE INDEX idx_sep12_customers_status ON offramp_service.sep12_customers (status);

CREATE TABLE offramp_service.sep12_customer_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    customer_id UUID NOT NULL REFERENCES offramp_service.sep12_customers (id),
    file_name TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size BIGINT NOT NULL,
    storage_path TEXT NOT NULL,
    purpose TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
);

CREATE INDEX idx_sep12_customer_files_customer ON offramp_service.sep12_customer_files (customer_id);

CREATE TABLE offramp_service.sep38_assets (
    id SERIAL PRIMARY KEY,
    asset TEXT NOT NULL UNIQUE,
    sell_delivery_methods JSONB,
    buy_delivery_methods JSONB,
    country_codes JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
);

CREATE INDEX idx_sep38_assets_asset ON offramp_service.sep38_assets (asset);

CREATE TABLE offramp_service.sep38_quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    original_quote_id TEXT NOT NULL UNIQUE,
    sell_asset TEXT NOT NULL,
    buy_asset TEXT NOT NULL,
    sell_amount NUMERIC NOT NULL,
    buy_amount NUMERIC NOT NULL,
    price NUMERIC NOT NULL,
    total_price NUMERIC NOT NULL,
    fee_total NUMERIC NOT NULL,
    fee_asset TEXT NOT NULL,
    fee_details JSONB,
    sell_delivery_method TEXT,
    buy_delivery_method TEXT,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    context TEXT NOT NULL,
    transaction_id UUID
);

CREATE INDEX idx_sep38_quotes_transaction ON offramp_service.sep38_quotes (transaction_id);

CREATE TABLE offramp_service.sep6_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    transaction_id TEXT NOT NULL UNIQUE,
    kind TEXT NOT NULL,
    status TEXT NOT NULL,
    status_eta BIGINT,
    more_info_url TEXT,
    amount_in NUMERIC,
    amount_in_asset TEXT,
    amount_out NUMERIC,
    amount_out_asset TEXT,
    amount_fee NUMERIC,
    amount_fee_asset TEXT,
    fee_details TEXT,
    quote_id TEXT,
    "from" TEXT,
    "to" TEXT,
    external_extra TEXT,
    external_extra_text TEXT,
    deposit_memo TEXT,
    deposit_memo_type TEXT,
    withdraw_anchor_account TEXT,
    withdraw_memo TEXT,
    withdraw_memo_type TEXT,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    user_action_required_by TIMESTAMP,
    stellar_transaction_id TEXT,
    external_transaction_id TEXT,
    message TEXT,
    refunded BOOLEAN,
    refunds TEXT,
    required_info_message TEXT,
    required_info_updates TEXT,
    instructions TEXT,
    claimable_balance_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP
);

CREATE INDEX idx_sep6_transactions_transaction_id ON offramp_service.sep6_transactions (transaction_id);

CREATE INDEX idx_sep6_transactions_status ON offramp_service.sep6_transactions (status);

CREATE TABLE offramp_service.sep6_refunds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    transaction_id UUID NOT NULL REFERENCES offramp_service.sep6_transactions (id),
    amount_refunded NUMERIC NOT NULL,
    amount_fee NUMERIC NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW ()
);

CREATE INDEX idx_sep6_refunds_transaction ON offramp_service.sep6_refunds (transaction_id);

CREATE TABLE offramp_service.sep6_refund_payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    refund_id UUID NOT NULL REFERENCES offramp_service.sep6_refunds (id),
    payment_id TEXT NOT NULL,
    id_type TEXT NOT NULL,
    amount NUMERIC NOT NULL,
    fee NUMERIC NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW ()
);

CREATE INDEX idx_sep6_refund_payments_refund ON offramp_service.sep6_refund_payments (refund_id);

CREATE TABLE offramp_service.sep31_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    account TEXT NOT NULL,
    memo TEXT,
    memo_type TEXT,
    transaction_id TEXT NOT NULL,
    amount NUMERIC NOT NULL,
    asset_code TEXT NOT NULL,
    asset_issuer TEXT,
    destination_asset TEXT,
    quote_id TEXT,
    sender_id TEXT NOT NULL,
    receiver_id TEXT NOT NULL,
    stellar_account_id TEXT,
    stellar_memo_type TEXT,
    stellar_memo TEXT,
    status TEXT NOT NULL,
    status_eta BIGINT,
    status_message TEXT,
    amount_in NUMERIC,
    amount_in_asset TEXT,
    amount_out NUMERIC,
    amount_out_asset TEXT,
    amount_fee NUMERIC,
    amount_fee_asset TEXT,
    fee_details JSONB,
    started_at TIMESTAMP,
    updated_at TIMESTAMP,
    completed_at TIMESTAMP,
    stellar_transaction_id TEXT,
    external_transaction_id TEXT,
    refunds JSONB,
    required_info_message TEXT,
    required_info_updates JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT NOW ()
);

CREATE INDEX idx_sep31_transactions_transaction_id ON offramp_service.sep31_transactions (transaction_id);

CREATE INDEX idx_sep31_transactions_status ON offramp_service.sep31_transactions (status);

CREATE TABLE offramp_service.sep24_withdrawals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    transaction_id VARCHAR NOT NULL,
    asset_code VARCHAR(255) NOT NULL,
    asset_issuer VARCHAR(255),
    amount NUMERIC,
    account VARCHAR(56),
    memo TEXT,
    memo_type VARCHAR(10),
    status VARCHAR(255) NOT NULL,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    stellar_transaction_id VARCHAR(255),
    external_transaction_id VARCHAR(255),
    quote_id VARCHAR(255),
    withdraw_anchor_account VARCHAR(255),
    withdraw_memo VARCHAR(255),
    withdraw_memo_type VARCHAR(10),
    wallet_name VARCHAR(255),
    wallet_url VARCHAR(255),
    lang VARCHAR(10),
    refund_memo VARCHAR(255),
    refund_memo_type VARCHAR(10),
    created_at TIMESTAMP NOT NULL
);
