// @generated automatically by Diesel CLI.

diesel::table! {
    campaign_lists (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        title -> Char,
        user_id -> Integer,
        queries -> Longtext,
        primary_entity -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 36]
        uuid -> Char,
    }
}

diesel::table! {
    campaign_rules (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 36]
        uuid -> Char,
        title -> Text,
        rules -> Longtext,
        campaign_template_id -> Integer,
        #[max_length = 255]
        initiation_type -> Char,
        #[max_length = 255]
        interval -> Nullable<Char>,
        #[max_length = 255]
        interval_period -> Nullable<Char>,
        initiation_date -> Datetime,
        admin_user_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        broadcast_option -> Nullable<Text>,
        broadcast_to -> Nullable<Longtext>,
        #[max_length = 255]
        slug -> Char,
        sent -> Integer,
        opened -> Integer,
    }
}

diesel::table! {
    campaign_templates (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 36]
        uuid -> Char,
        title -> Text,
        subject -> Nullable<Text>,
        sender -> Nullable<Text>,
        message -> Nullable<Longtext>,
        variables -> Nullable<Longtext>,
        admin_user_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        greeting -> Nullable<Char>,
        push_title -> Nullable<Longtext>,
        push_message -> Nullable<Longtext>,
        push_image -> Nullable<Longtext>,
        push_url -> Nullable<Longtext>,
        #[max_length = 255]
        label -> Nullable<Char>,
        mail_content -> Nullable<Longtext>,
        push_content -> Nullable<Longtext>,
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
    notifications (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 36]
        uuid -> Char,
        title -> Text,
        body -> Longtext,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Char,
        #[max_length = 255]
        model_type -> Nullable<Char>,
        user_id -> Nullable<Integer>,
        #[max_length = 255]
        model_type_id -> Nullable<Char>,
        extra_url -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        unread -> Bool,
    }
}

diesel::table! {
    push_notifications (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 36]
        uuid -> Char,
        #[max_length = 255]
        device_type -> Char,
        user_id -> Integer,
        device_token -> Longtext,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    triggered_campaigns (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 36]
        uuid -> Char,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Char,
        user_id -> Integer,
        campaign_template_id -> Integer,
        campaign_rule_id -> Integer,
        sent -> Bool,
        opened -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        campaign_step_id -> Char,
        #[max_length = 255]
        entity_uuid -> Char,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    campaign_lists,
    campaign_rules,
    campaign_templates,
    migrations,
    notifications,
    push_notifications,
    triggered_campaigns,
);
