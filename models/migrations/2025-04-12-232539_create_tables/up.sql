-- Accounts Table
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

-- SEP-6 Transactions Table
CREATE TABLE sep6_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id TEXT NOT NULL,
    kind TEXT NOT NULL, -- "withdrawal" or "withdrawal-exchange"
    status TEXT NOT NULL,
    status_eta BIGINT,
    more_info_url TEXT,
    amount_in NUMERIC(20, 8),
    amount_in_asset TEXT,
    amount_out NUMERIC(20, 8),
    amount_out_asset TEXT,
    amount_fee NUMERIC(20, 8),
    amount_fee_asset TEXT,
    quote_id TEXT,
    account TEXT NOT NULL,
    memo TEXT,
    memo_type TEXT,
    withdraw_anchor_account TEXT,
    withdraw_memo TEXT,
    withdraw_memo_type TEXT,
    external_transaction_id TEXT,
    stellar_transaction_id TEXT,
    refunded BOOLEAN,
    required_info_updates JSONB,
    required_info_message TEXT,
    claimable_balance_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    user_action_required_by TIMESTAMP
);

-- SEP-12 Customers Table
CREATE TABLE sep12_customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account TEXT NOT NULL,
    memo TEXT,
    memo_type TEXT,
    customer_type TEXT NOT NULL,
    status TEXT NOT NULL,
    first_name TEXT,
    last_name TEXT,
    email TEXT,
    phone TEXT,
    date_of_birth DATE,
    address_street TEXT,
    address_city TEXT,
    address_state TEXT,
    address_postal_code TEXT,
    address_country TEXT,
    kyc_verified BOOLEAN NOT NULL DEFAULT false,
    verification_status TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_verified_at TIMESTAMP
);

-- SEP-12 Customer Files Table
CREATE TABLE sep12_customer_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES sep12_customers(id) ON DELETE CASCADE,
    file_name TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size BIGINT NOT NULL,
    storage_path TEXT NOT NULL,
    purpose TEXT NOT NULL,
    uploaded_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP
);

-- SEP-12 Transactions Table
CREATE TABLE sep12_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id TEXT NOT NULL,
    account TEXT NOT NULL,
    memo TEXT,
    memo_type TEXT,
    customer_id UUID NOT NULL REFERENCES sep12_customers(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    required_fields JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- SEP-12 Callbacks Table
CREATE TABLE sep12_callbacks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account TEXT NOT NULL,
    url TEXT NOT NULL,
    last_attempt TIMESTAMP,
    last_status TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- SEP-12 Verifications Table
CREATE TABLE sep12_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES sep12_customers(id) ON DELETE CASCADE,
    method TEXT NOT NULL,
    status TEXT NOT NULL,
    verified_at TIMESTAMP,
    expires_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- SEP-38 Assets Table
CREATE TABLE sep38_assets (
    id SERIAL PRIMARY KEY,
    asset TEXT NOT NULL UNIQUE,
    sell_delivery_methods JSONB,
    buy_delivery_methods JSONB,
    country_codes JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- SEP-38 Quotes Table
CREATE TABLE sep38_quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_quote_id TEXT NOT NULL,
    sell_asset TEXT NOT NULL,
    buy_asset TEXT NOT NULL,
    sell_amount NUMERIC(20, 8) NOT NULL,
    buy_amount NUMERIC(20, 8) NOT NULL,
    price NUMERIC(20, 8) NOT NULL,
    total_price NUMERIC(20, 8) NOT NULL,
    fee_total NUMERIC(20, 8) NOT NULL,
    fee_asset TEXT NOT NULL,
    fee_details JSONB,
    sell_delivery_method TEXT,
    buy_delivery_method TEXT,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    context TEXT NOT NULL,
    transaction_id UUID
);

-- Offramp Transactions Table
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

-- Offramp Quotes Table
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

-- Indexes
CREATE INDEX idx_sep6_transactions_account ON sep6_transactions(account);
CREATE INDEX idx_sep6_transactions_transaction_id ON sep6_transactions(transaction_id);
CREATE INDEX idx_sep12_customers_account ON sep12_customers(account);
CREATE INDEX idx_sep12_customers_kyc_status ON sep12_customers(kyc_verified, verification_status);
CREATE INDEX idx_sep12_customer_files_customer_id ON sep12_customer_files(customer_id);
CREATE INDEX idx_sep12_transactions_customer_id ON sep12_transactions(customer_id);
CREATE INDEX idx_sep12_transactions_transaction_id ON sep12_transactions(transaction_id);
CREATE INDEX idx_sep12_callbacks_account ON sep12_callbacks(account);
CREATE INDEX idx_sep38_quotes_original_quote_id ON sep38_quotes(original_quote_id);
CREATE INDEX idx_sep38_quotes_transaction_id ON sep38_quotes(transaction_id);
CREATE INDEX idx_offramp_quotes_transaction_id ON offramp_quotes(transaction_id);