pub mod user {
    use crate::auth::{models::role::role::Role, schema::users};
    use diesel::{mysql::Mysql, prelude::*};
    use serde::{Deserialize, Serialize};

    #[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Identifiable, Associations)]
    #[diesel(belongs_to(Role, foreign_key = role_id))]
    #[diesel(table_name = users)]
    #[diesel(check_for_backend(Mysql))]
    pub struct User {
        pub id: u64,
        pub uuid: String,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub full_name: Option<String>,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub email_verified_at: Option<chrono::NaiveDateTime>,
        pub password: Option<String>,
        pub password_created_at: Option<chrono::NaiveDateTime>,
        pub is_login_email: bool,
        pub phone_verified_at: Option<chrono::NaiveDateTime>,
        pub status: String,
        pub otp: Option<u64>,
        pub otp_expired_at: Option<chrono::NaiveDateTime>,
        pub role_id: i32,
        pub created_at: Option<chrono::NaiveDateTime>,
        pub updated_at: Option<chrono::NaiveDateTime>,
        pub deleted_at: Option<chrono::NaiveDateTime>,
        pub username: Option<String>,
    }

    #[derive(Insertable, Debug)]
    #[diesel(table_name = users)]
    pub struct NewUser<'a> {
        pub uuid: &'a str,
        pub first_name: Option<&'a str>,
        pub last_name: Option<&'a str>,
        pub full_name: Option<&'a str>,
        pub email: Option<&'a str>,
        pub phone: Option<&'a str>,
        pub email_verified_at: Option<chrono::NaiveDateTime>,
        pub password: Option<&'a str>,
        pub password_created_at: Option<chrono::NaiveDateTime>,
        pub is_login_email: bool,
        pub phone_verified_at: Option<chrono::NaiveDateTime>,
        pub status: &'a str,
        pub otp: Option<u64>,
        pub otp_expired_at: Option<chrono::NaiveDateTime>,
        pub role_id: i32,
        pub created_at: Option<chrono::NaiveDateTime>,
        pub updated_at: Option<chrono::NaiveDateTime>,
        pub deleted_at: Option<chrono::NaiveDateTime>,
        pub username: Option<&'a str>,
    }

    #[derive(AsChangeset)]
    #[diesel(table_name = users)]
    pub struct UpdateUser<'a> {
        pub first_name: Option<&'a str>,
        pub last_name: Option<&'a str>,
        pub full_name: Option<&'a str>,
        pub email: Option<&'a str>,
        pub phone: Option<&'a str>,
        pub email_verified_at: Option<chrono::NaiveDateTime>,
        pub password: Option<&'a str>,
        pub password_created_at: Option<chrono::NaiveDateTime>,
        pub is_login_email: Option<bool>,
        pub phone_verified_at: Option<chrono::NaiveDateTime>,
        pub status: Option<&'a str>,
        pub otp: Option<u64>,
        pub otp_expired_at: Option<chrono::NaiveDateTime>,
        pub role_id: Option<i32>,
        pub created_at: Option<chrono::NaiveDateTime>,
        pub updated_at: Option<chrono::NaiveDateTime>,
        pub deleted_at: Option<chrono::NaiveDateTime>,
        pub username: Option<&'a str>,
    }
}
