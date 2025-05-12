// @generated automatically by Diesel CLI.

pub mod offramp_service {
    diesel::table! {
        offramp_service.migrations (id) {
            id -> Unsigned<Integer>,
            #[max_length = 255]
            migration -> Varchar,
            batch -> Integer,
        }
    }

    diesel::table! {
        offramp_service.sep12_customers (id) {
            id -> Uuid,
            #[max_length = 56]
            account -> Varchar,
            memo -> Nullable<Text>,
            #[max_length = 10]
            memo_type -> Nullable<Varchar>,
            #[max_length = 20]
            customer_type -> Varchar,
            #[max_length = 20]
            status -> Varchar,
            #[max_length = 255]
            first_name -> Nullable<Varchar>,
            #[max_length = 255]
            last_name -> Nullable<Varchar>,
            #[max_length = 255]
            email -> Nullable<Varchar>,
            #[max_length = 255]
            phone -> Nullable<Varchar>,
            #[max_length = 255]
            date_of_birth -> Nullable<Varchar>,
            #[max_length = 255]
            address_street -> Nullable<Varchar>,
            #[max_length = 255]
            address_city -> Nullable<Varchar>,
            #[max_length = 255]
            address_state -> Nullable<Varchar>,
            #[max_length = 255]
            address_postal_code -> Nullable<Varchar>,
            #[max_length = 255]
            address_country -> Nullable<Varchar>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }

    diesel::table! {
        offramp_service.sep12_customer_files (id) {
            id -> Uuid,
            customer_id -> Uuid,
            #[max_length = 255]
            file_name -> Varchar,
            #[max_length = 255]
            content_type -> Varchar,
            size -> Bigint,
            #[max_length = 255]
            storage_path -> Varchar,
            #[max_length = 255]
            purpose -> Varchar,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }

    diesel::table! {
        offramp_service.sep38_assets (id) {
            id -> Integer,
            #[max_length = 255]
            asset -> Varchar,
            sell_delivery_methods -> Nullable<Json>,
            buy_delivery_methods -> Nullable<Json>,
            country_codes -> Nullable<Json>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }

    diesel::table! {
        offramp_service.sep38_quotes (id) {
            id -> Uuid,
            #[max_length = 255]
            original_quote_id -> Varchar,
            #[max_length = 255]
            sell_asset -> Varchar,
            #[max_length = 255]
            buy_asset -> Varchar,
            sell_amount -> Numeric,
            buy_amount -> Numeric,
            price -> Numeric,
            total_price -> Numeric,
            fee_total -> Numeric,
            #[max_length = 255]
            fee_asset -> Varchar,
            fee_details -> Nullable<Json>,
            #[max_length = 255]
            sell_delivery_method -> Nullable<Varchar>,
            #[max_length = 255]
            buy_delivery_method -> Nullable<Varchar>,
            expires_at -> Timestamp,
            created_at -> Timestamp,
            #[max_length = 255]
            context -> Varchar,
            transaction_id -> Nullable<Uuid>,
        }
    }

    diesel::table! {
        offramp_service.sep6_refund_payments (id) {
            id -> Uuid,
            refund_id -> Uuid,
            #[max_length = 255]
            payment_id -> Varchar,
            #[max_length = 255]
            id_type -> Varchar,
            amount -> Numeric,
            fee -> Numeric,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        offramp_service.sep6_refunds (id) {
            id -> Uuid,
            transaction_id -> Uuid,
            amount_refunded -> Numeric,
            amount_fee -> Numeric,
            created_at -> Timestamp,
        }
    }
    diesel::table! {
        #[sql_name = "sep6_transactions"]
        #[allow(non_snake_case)]
        offramp_service.sep6_transactions (id) {
            id -> Uuid,
            #[max_length = 255]
            anchor_slug -> Varchar,
            #[max_length = 255]
            transaction_id -> Varchar,
            #[max_length = 255]
            kind -> Varchar,
            #[max_length = 255]
            status -> Varchar,
            status_eta -> Nullable<Bigint>,
            #[max_length = 255]
            more_info_url -> Nullable<Varchar>,
            amount_in -> Nullable<Numeric>,
            #[max_length = 255]
            amount_in_asset -> Nullable<Varchar>,
            amount_out -> Nullable<Numeric>,
            #[max_length = 255]
            amount_out_asset -> Nullable<Varchar>,
            amount_fee -> Nullable<Numeric>,
            #[max_length = 255]
            amount_fee_asset -> Nullable<Varchar>,
            #[max_length = 255]
            quote_id -> Nullable<Varchar>,
            #[max_length = 255]
            from -> Nullable<Varchar>,
            #[max_length = 255]
            to -> Nullable<Varchar>,
            external_extra -> Nullable<Text>,
            external_extra_text -> Nullable<Text>,
            #[max_length = 255]
            deposit_memo -> Nullable<Varchar>,
            #[max_length = 255]
            deposit_memo_type -> Nullable<Varchar>,
            #[max_length = 255]
            withdraw_anchor_account -> Nullable<Varchar>,
            #[max_length = 255]
            fee_details-> Nullable<Varchar>,
            #[max_length = 255]
            withdraw_memo -> Nullable<Varchar>,
            #[max_length = 255]
            withdraw_memo_type -> Nullable<Varchar>,
            started_at -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
            completed_at -> Nullable<Timestamp>,
            user_action_required_by -> Nullable<Timestamp>,
            #[max_length = 255]
            stellar_transaction_id -> Nullable<Varchar>,
            #[max_length = 255]
            external_transaction_id -> Nullable<Varchar>,
            message -> Nullable<Text>,
            refunded -> Nullable<Bool>,
            required_info_message -> Nullable<Varchar>,
            required_info_updates -> Nullable<Text>,
            instructions -> Nullable<Text>,
            refunds -> Nullable<Varchar>,
            claimable_balance_id -> Nullable<Varchar>,
            created_at -> Timestamp,

        }
    }

    diesel::table! {
        offramp_service.user_auth_tokens (id) {
            id -> Unsigned<Bigint>,
            #[max_length = 255]
            auth_id -> Char,
            auth_token -> Longtext,
            created_at -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        offramp_service.users (id) {
            id -> Unsigned<Bigint>,
            #[max_length = 36]
            uuid -> Char,
            #[max_length = 255]
            first_name -> Nullable<Char>,
            #[max_length = 255]
            last_name -> Nullable<Char>,
            #[max_length = 255]
            full_name -> Nullable<Char>,
            #[max_length = 255]
            email -> Nullable<Char>,
            #[max_length = 255]
            phone -> Nullable<Char>,
            email_verified_at -> Nullable<Datetime>,
            #[max_length = 255]
            password -> Nullable<Varchar>,
            password_created_at -> Nullable<Datetime>,
            is_login_email -> Bool,
            phone_verified_at -> Nullable<Datetime>,
            #[max_length = 255]
            status -> Char,
            otp -> Nullable<Unsigned<Bigint>>,
            otp_expired_at -> Nullable<Datetime>,
            role_id -> Integer,
            created_at -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
            deleted_at -> Nullable<Timestamp>,
            #[max_length = 255]
            username -> Nullable<Char>,
        }
    }

    diesel::joinable!(sep12_customer_files -> sep12_customers (customer_id));
    diesel::joinable!(sep6_refund_payments -> sep6_refunds (refund_id));

    diesel::allow_tables_to_appear_in_same_query!(
        sep12_customers,
        sep12_customer_files,
        sep38_assets,
        sep38_quotes,
        sep6_refund_payments,
        sep6_refunds,
        sep6_transactions,
        user_auth_tokens,
        users,
    );
}
