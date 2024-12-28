use chrono::NaiveDateTime;
use form::form::{CreateUserForm, UpdateUserForm};
use models::auth::models::user::user::{NewUser, UpdateUser, User};
use rocket::form::Form;
use uuid::Uuid;

pub mod form;

fn parse_naive_datetime(input: Option<&str>) -> Option<NaiveDateTime> {
    input.and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok())
}

pub async fn get_many_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
    Ok(services::auth::auth::get_many_users().await?)
}

pub async fn create_new_user(
    data: Form<CreateUserForm<'_>>,
) -> Result<User, Box<dyn std::error::Error>> {
    let unique_uuid = Uuid::new_v4().to_string();
    let unique_uuid_str = unique_uuid.as_str();

    let new_user = NewUser {
        uuid: &unique_uuid_str,
        first_name: data.first_name,
        last_name: data.last_name,
        full_name: data.full_name,
        email: Some(data.email),
        phone: data.phone,
        email_verified_at: parse_naive_datetime(data.email_verified_at),
        password: Some(data.password),
        password_created_at: parse_naive_datetime(data.password_created_at),
        is_login_email: data.is_login_email,
        phone_verified_at: parse_naive_datetime(data.phone_verified_at),
        status: data.status,
        otp: data.otp,
        otp_expired_at: parse_naive_datetime(data.otp_expired_at),
        role_id: data.role_id,
        created_at: parse_naive_datetime(data.created_at),
        updated_at: parse_naive_datetime(data.updated_at),
        deleted_at: parse_naive_datetime(data.deleted_at),
        username: data.username,
    };

    Ok(services::auth::auth::create_user(new_user).await?)
}

pub async fn update_user(
    uuid: &str,
    data: Form<UpdateUserForm<'_>>,
) -> Result<User, Box<dyn std::error::Error>> {
    let user_update = UpdateUser {
        first_name: data.first_name,
        last_name: data.last_name,
        full_name: data.full_name,
        email: data.email,
        phone: data.phone,
        email_verified_at: parse_naive_datetime(data.email_verified_at),
        password: data.password,
        password_created_at: parse_naive_datetime(data.password_created_at),
        is_login_email: data.is_login_email,
        phone_verified_at: parse_naive_datetime(data.phone_verified_at),
        status: data.status,
        otp: data.otp,
        otp_expired_at: parse_naive_datetime(data.otp_expired_at),
        role_id: data.role_id,
        created_at: parse_naive_datetime(data.created_at),
        updated_at: parse_naive_datetime(data.updated_at),
        deleted_at: parse_naive_datetime(data.deleted_at),
        username: data.username,
    };

    Ok(services::auth::auth::update_user(uuid, user_update).await?)
}

pub async fn delete_user(uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(services::auth::auth::delete_user(uuid).await?)
}
