diesel::table! {
    offramp_transactions (id) {
        id -> Uuid,
        account_id -> Uuid,
        transaction_id -> Text,
        amount -> Numeric,
        dest_currency -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    offramp_quotes (id) {
        id -> Uuid,
        transaction_id -> Uuid,
        quote_id -> Text,
        sell_asset -> Text,
        buy_asset -> Text,
        sell_amount -> Numeric,
        buy_amount -> Numeric,
        price -> Numeric,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}



diesel::table! {
    asset_info (id) {
        id -> Uuid,
        asset_code -> Text,
        asset_issuer -> Nullable<Text>,
        operation_type -> Text,
        min_amount -> Nullable<Double>,
        max_amount -> Nullable<Double>,
        fee_fixed -> Nullable<Double>,
        fee_percent -> Nullable<Double>,
        sep12_sender_fields -> Nullable<Jsonb>,
        sep12_receiver_fields -> Nullable<Jsonb>,
        sep38_contexts -> Nullable<Array<Text>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}



diesel::joinable!(offramp_quotes -> offramp_transactions (transaction_id));
diesel::joinable!(offramp_transactions -> accounts (account_id));
diesel::joinable!(payment_errors -> payments (payment_id));
diesel::joinable!(payments -> accounts (source_account_id));
diesel::joinable!(payments -> accounts (destination_account_id));

// Add to the allow_tables_to_appear_in_same_query! macro:
diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    encrypted_keys,
    tokens,
    transaction_errors,
    transactions,
    trustlines,
    offramp_transactions,
    offramp_quotes,
    payments,
    payment_errors,
);