pub mod form {
    use rocket::form::FromForm;

    #[derive(FromForm)]
    pub struct CreateUserForm<'r> {
        pub uuid: &'r str,
        pub first_name: Option<&'r str>,
        pub last_name: Option<&'r str>,
        pub full_name: Option<&'r str>,
        pub email: &'r str,
        pub phone: Option<&'r str>,
        pub email_verified_at: Option<&'r str>,
        pub password: &'r str,
        pub password_created_at: Option<&'r str>,
        pub is_login_email: bool,
        pub phone_verified_at: Option<&'r str>,
        pub status: &'r str,
        pub otp: Option<u64>,
        pub otp_expired_at: Option<&'r str>,
        pub role_id: i32,
        pub created_at: Option<&'r str>,
        pub updated_at: Option<&'r str>,
        pub deleted_at: Option<&'r str>,
        pub username: Option<&'r str>,
    }

    #[derive(FromForm)]
    pub struct UpdateUserForm<'r> {
        pub first_name: Option<&'r str>,
        pub last_name: Option<&'r str>,
        pub full_name: Option<&'r str>,
        pub email: Option<&'r str>,
        pub phone: Option<&'r str>,
        pub email_verified_at: Option<&'r str>,
        pub password: Option<&'r str>,
        pub password_created_at: Option<&'r str>,
        pub is_login_email: Option<bool>,
        pub phone_verified_at: Option<&'r str>,
        pub status: Option<&'r str>,
        pub otp: Option<u64>,
        pub otp_expired_at: Option<&'r str>,
        pub role_id: Option<i32>,
        pub created_at: Option<&'r str>,
        pub updated_at: Option<&'r str>,
        pub deleted_at: Option<&'r str>,
        pub username: Option<&'r str>,
    }
}
