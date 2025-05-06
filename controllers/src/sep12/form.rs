pub mod form {
    use rocket::form::FromForm;


    #[derive(FromForm)]
    pub struct Sep12KycStatusForm<'r> {
        pub slug: &'r str,
        pub account: &'r str,
        pub memo: Option<&'r str>,
        pub customer_type: Option<&'r str>,
    }

    #[derive(FromForm)]
    pub struct Sep12CreateKycForm<'r> {
       pub slug: &'r str,
        pub account: &'r str,
        pub memo: Option<&'r str>,
        pub customer_type: &'r str,
       
    }

    #[derive(FromForm)]
    pub struct Sep12UpdateKycForm<'r> {
        pub slug: &'r str,
        pub customer_id: &'r str,
     
    }
    
    #[derive(FromForm)]
    pub struct Sep12DeleteKycForm<'r> {
       pub slug: &'r str,
        pub account: &'r str,
        pub memo: Option<&'r str>,
    }

    #[derive(FromForm)]
    pub struct Sep12RequiredVerificationForm<'r> {
        pub slug: &'r str,
        pub account: &'r str,
        pub memo: Option<&'r str>,
        pub customer_type: Option<&'r str>,
    }
}