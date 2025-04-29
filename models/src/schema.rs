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
    
        sep38_assets (id) {
            id -> Int4,
            asset -> Text,
            sell_delivery_methods -> Nullable<Jsonb>,
            buy_delivery_methods -> Nullable<Jsonb>,
            country_codes -> Nullable<Jsonb>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep38_quotes (id) {
            id -> Uuid,
            original_quote_id -> Text,
            sell_asset -> Text,
            buy_asset -> Text,
            sell_amount -> Numeric,
            buy_amount -> Numeric,
            price -> Numeric,
            total_price -> Numeric,
            fee_total -> Numeric,
            fee_asset -> Text,
            fee_details -> Nullable<Jsonb>,
            sell_delivery_method -> Nullable<Text>,
            buy_delivery_method -> Nullable<Text>,
            expires_at -> Timestamp,
            created_at -> Timestamp,
            context -> Text,
            transaction_id -> Nullable<Uuid>,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
        
        price_responses (id) {
            id -> Uuid,
            sell_asset -> Text,
            buy_asset -> Text,
            price -> Numeric,
            total_price -> Numeric,
            sell_amount -> Nullable<Numeric>,
            buy_amount -> Nullable<Numeric>,
            fee_details -> Nullable<Jsonb>,
            fee_total -> Nullable<Numeric>,
            fee_asset -> Nullable<Text>,
            expires_at -> Nullable<Timestamp>,
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
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep6_transactions (id) {
            id -> Uuid,
            transaction_id -> Text,
            kind -> Text,
            status -> Text,
            status_eta -> Nullable<Int8>,
            more_info_url -> Nullable<Text>,
            amount_in -> Nullable<Numeric>,
            amount_in_asset -> Nullable<Text>,
            amount_out -> Nullable<Numeric>,
            amount_out_asset -> Nullable<Text>,
            amount_fee -> Nullable<Numeric>,
            amount_fee_asset -> Nullable<Text>,
            quote_id -> Nullable<Text>,
            account -> Text,
            memo -> Nullable<Text>,
            memo_type -> Nullable<Text>,
            withdraw_anchor_account -> Nullable<Text>,
            withdraw_memo -> Nullable<Text>,
            withdraw_memo_type -> Nullable<Text>,
            external_transaction_id -> Nullable<Text>,
            stellar_transaction_id -> Nullable<Text>,
            refunded -> Nullable<Bool>,
            required_info_updates -> Nullable<Jsonb>,
            required_info_message -> Nullable<Text>,
            claimable_balance_id -> Nullable<Text>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
            started_at -> Nullable<Timestamp>,
            completed_at -> Nullable<Timestamp>,
            user_action_required_by -> Nullable<Timestamp>,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep6_fees (id) {
            id -> Uuid,
            asset_code -> Text,
            operation_type -> Text,
            fee_fixed -> Nullable<Numeric>,
            fee_percent -> Nullable<Numeric>,
            fee_min -> Nullable<Numeric>,
            fee_max -> Nullable<Numeric>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep6_withdraw_methods (id) {
            id -> Uuid,
            asset_code -> Text,
            method_name -> Text,
            description -> Nullable<Text>,
            fields_required -> Nullable<Jsonb>,
            min_amount -> Nullable<Numeric>,
            max_amount -> Nullable<Numeric>,
            fee_fixed -> Nullable<Numeric>,
            fee_percent -> Nullable<Numeric>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep12_customers (id) {
            id -> Uuid,
            account -> Text,
            memo -> Nullable<Text>,
            memo_type -> Nullable<Text>,
            customer_type -> Text,
            status -> Text,
            first_name -> Nullable<Text>,
            last_name -> Nullable<Text>,
            email -> Nullable<Text>,
            phone -> Nullable<Text>,
            date_of_birth -> Nullable<Date>,
            address_street -> Nullable<Text>,
            address_city -> Nullable<Text>,
            address_state -> Nullable<Text>,
            address_postal_code -> Nullable<Text>,
            address_country -> Nullable<Text>,
            kyc_verified -> Bool,
            verification_status -> Nullable<Text>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
            last_verified_at -> Nullable<Timestamp>,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep12_customer_files (id) {
            id -> Uuid,
            customer_id -> Uuid,
            file_name -> Text,
            content_type -> Text,
            size -> BigInt,
            storage_path -> Text,
            purpose -> Text,
            uploaded_at -> Timestamp,
            expires_at -> Nullable<Timestamp>,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep12_transactions (id) {
            id -> Uuid,
            transaction_id -> Text,
            account -> Text,
            memo -> Nullable<Text>,
            memo_type -> Nullable<Text>,
            customer_id -> Uuid,
            status -> Text,
            required_fields -> Nullable<Jsonb>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep12_callbacks (id) {
            id -> Uuid,
            account -> Text,
            url -> Text,
            last_attempt -> Nullable<Timestamp>,
            last_status -> Nullable<Text>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    
    diesel::table! {
        use diesel::sql_types::*;
    
        sep12_verifications (id) {
            id -> Uuid,
            customer_id -> Uuid,
            method -> Text,
            status -> Text,
            verified_at -> Nullable<Timestamp>,
            expires_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    
    diesel::joinable!(offramp_quotes -> offramp_transactions (transaction_id));
    diesel::joinable!(offramp_transactions -> accounts (account_id));
    diesel::joinable!(sep38_quotes -> sep6_transactions (transaction_id));
    diesel::joinable!(sep12_customer_files -> sep12_customers (customer_id));
    diesel::joinable!(sep12_transactions -> sep12_customers (customer_id));
    diesel::joinable!(sep12_verifications -> sep12_customers (customer_id));
    
    diesel::allow_tables_to_appear_in_same_query!(
        accounts,
        asset_info,
        offramp_transactions,
        offramp_quotes,
        sep6_transactions,
        sep6_fees,
        sep6_withdraw_methods,
        sep38_assets,
        sep38_quotes,
        sep12_customers,
        sep12_customer_files,
        sep12_transactions,
        sep12_callbacks,
        sep12_verifications,
    );
}