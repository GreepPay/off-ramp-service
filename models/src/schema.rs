// @generated automatically by Diesel CLI.
pub mod offramp_service {
    diesel::table! {
        use diesel::sql_types::*;
    
        accounts (id) {
            id -> Uuid,
            stellar_address -> Text,
            email -> Nullable<Text>,
            name -> Nullable<Text>,
            created_at -> Timestamp,
            updated_at -> Timestamp, 
            status -> Text,
            kyc_status -> Text,
            last_login -> Nullable<Timestamp>,
            last_kyc_submitted -> Nullable<Timestamp>,
            phone -> Nullable<Text>,
            balance -> Nullable<Numeric>,
            memo -> Nullable<Text>,
            memo_type -> Nullable<Text>,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
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
        use diesel::sql_types::*;
    
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
        use diesel::sql_types::*;
    
        asset_info (asset_code, operation_type) {
            asset_code -> Text,
            operation_type -> Text,
            asset_issuer -> Nullable<Text>,
            min_amount -> Nullable<Double>,
            max_amount -> Nullable<Double>,
            fee_fixed -> Nullable<Double>,
            fee_percent -> Nullable<Double>,
            sep12_fields -> Nullable<Jsonb>,
            sep38_contexts -> Nullable<Jsonb>,
            extra_fields -> Nullable<Jsonb>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    
    diesel::joinable!(offramp_quotes -> offramp_transactions (transaction_id));
    diesel::joinable!(offramp_transactions -> accounts (account_id));
    
    diesel::allow_tables_to_appear_in_same_query!(
        accounts,
        asset_info,
        offramp_transactions,
        offramp_quotes,
    );

}