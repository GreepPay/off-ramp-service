pub mod form {

    use rocket::form::FromForm;
    use rocket::fs::TempFile;
    use rocket::serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep12KycStatusForm {
        pub slug: String,
        pub account: String,
        #[serde(default)]
        pub memo: Option<String>,
        #[serde(default)]
        pub customer_type: Option<String>,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep12DeleteKycForm {
        pub slug: String,
        pub account: String,
        #[serde(default)]
        pub memo: Option<String>,
    }

    #[derive(FromForm)]
    pub struct Sep12FileField<'v> {
        #[field(name = "unused")]
        pub name: String,
        #[field(name = "file")]
        pub data: TempFile<'v>,
        #[field(name = "content_type")]
        pub content_type: String,
    }

    #[derive(FromForm)]
    pub struct Sep12FieldsAndFiles<'v> {
        // KYC fields
        pub slug: String,
        pub account: String,
        #[field(default = None)]
        pub memo: Option<String>,
        pub customer_type: String,
        #[field(name = "field")]
        pub fields: Vec<(String, String)>,
        #[field(name = "file_field")]
        pub files: Vec<Sep12FileField<'v>>,
    }

    #[derive(FromForm)]
    pub struct Sep12UpdateKycForm<'v> {
        pub slug: String,
        pub customer_id: String,
        #[field(name = "field")]
        pub fields: Vec<(String, String)>,
        #[field(name = "file_field")]
        pub files: Vec<Sep12FileField<'v>>,
        pub account: String,
    }
}
