// @generated automatically by Diesel CLI.

diesel::table! {
    failed_jobs (id) {
        id -> Unsigned<Integer>,
        connection -> Text,
        queue -> Text,
        payload -> Longtext,
        exception -> Longtext,
        failed_at -> Timestamp,
    }
}

diesel::table! {
    migrations (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        migration -> Varchar,
        batch -> Integer,
    }
}

diesel::table! {
    permissions (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 36]
        uuid -> Char,
        role_id -> Integer,
        status -> Bool,
        #[max_length = 255]
        name -> Char,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        key -> Char,
        #[max_length = 255]
        sub_key -> Char,
    }
}

diesel::table! {
    roles (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 36]
        uuid -> Char,
        #[max_length = 255]
        name -> Char,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        editable_name -> Nullable<Text>,
    }
}

diesel::table! {
    user_auth_tokens (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        auth_id -> Char,
        auth_token -> Longtext,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
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

diesel::allow_tables_to_appear_in_same_query!(
    failed_jobs,
    migrations,
    permissions,
    roles,
    user_auth_tokens,
    users,
);
