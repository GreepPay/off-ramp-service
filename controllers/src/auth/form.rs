pub mod form {
    use rocket::form::FromForm;

    #[derive(FromForm)]
    pub struct ChallengeRequestForm<'r> {
        pub account: &'r str,
        #[field(name = "client_domain")]
        pub client_domain: Option<&'r str>,
    }

    #[derive(FromForm)]
    pub struct TokenRequestForm<'r> {
        pub transaction: &'r str,
    }
}