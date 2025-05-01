pub mod form {
    use rocket::form::FromForm;
    use std::collections::HashMap;
    use services::kyc::{
        CustomerQuery, CustomerRequest, CustomerFields, 
        CallbackRequest, VerificationRequest, FileUpload, 
        UploadedFile, FileQuery
    };

    #[derive(FromForm)]
    pub struct CustomerQueryForm<'r> {
        pub account: &'r str,
        #[field(name = "id")]
        pub id: Option<&'r str>,
        pub memo: Option<&'r str>,
        #[field(name = "memo_type")]
        pub memo_type: Option<&'r str>,
        #[field(name = "type")]
        pub customer_type: Option<&'r str>,
        #[field(name = "transaction_id")]
        pub transaction_id: Option<&'r str>,
        pub lang: Option<&'r str>,
    }

    impl<'r> From<CustomerQueryForm<'r>> for CustomerQuery {
        fn from(form: CustomerQueryForm<'r>) -> Self {
            Self {
                account: form.account.to_string(),
                id: form.id.map(|s| s.to_string()),
                memo: form.memo.map(|s| s.to_string()),
                memo_type: form.memo_type.map(|s| s.to_string()),
                customer_type: form.customer_type.map(|s| s.to_string()),
                transaction_id: form.transaction_id.map(|s| s.to_string()),
                lang: form.lang.map(|s| s.to_string()),
            }
        }
    }

    #[derive(FromForm)]
    pub struct CustomerRequestForm<'r> {
        pub account: &'r str,
        pub memo: Option<&'r str>,
        #[field(name = "memo_type")]
        pub memo_type: Option<&'r str>,
        #[field(name = "type")]
        pub customer_type: Option<&'r str>,
        #[field(name = "fields")]
        pub fields: Option<String>,
    }

    impl<'r> From<CustomerRequestForm<'r>> for CustomerRequest {
        fn from(form: CustomerRequestForm<'r>) -> Self {
            Self {
                account: form.account.to_string(),
                memo: form.memo.map(|s| s.to_string()),
                memo_type: form.memo_type.map(|s| s.to_string()),
                customer_type: form.customer_type.map(|s| s.to_string()),
                fields: form.fields.map_or_else(
                    || CustomerFields::Text(HashMap::new()),
                    |v| {
                        let fields: HashMap<String, String> = serde_json::from_str(&v)
                            .unwrap_or_default();
                        CustomerFields::Text(fields)
                    }
                ),
            }
        }
    }

    #[derive(FromForm)]
    pub struct CallbackRequestForm<'r> {
        pub account: &'r str,
        pub id: Option<&'r str>,
        pub memo: Option<&'r str>,
        #[field(name = "memo_type")]
        pub memo_type: Option<&'r str>,
        pub url: &'r str,
    }

    impl<'r> From<CallbackRequestForm<'r>> for CallbackRequest {
        fn from(form: CallbackRequestForm<'r>) -> Self {
            Self {
                account: form.account.to_string(),
                id: form.id.map(|s| s.to_string()),
                memo: form.memo.map(|s| s.to_string()),
                memo_type: form.memo_type.map(|s| s.to_string()),
                url: form.url.to_string(),
            }
        }
    }

    #[derive(FromForm)]
    pub struct VerificationRequestForm<'r> {
        pub id: &'r str,
        #[field(name = "fields")]
        pub fields: Option<String>,
    }

    impl<'r> From<VerificationRequestForm<'r>> for VerificationRequest {
        fn from(form: VerificationRequestForm<'r>) -> Self {
            Self {
                id: form.id.to_string(),
                fields: form.fields.map_or_else(
                    || HashMap::new(),
                    |v| serde_json::from_str(&v).unwrap_or_default()
                ),
            }
        }
    }

    #[derive(FromForm)]
    pub struct FileUploadForm<'r> {
        pub account: &'r str,
        pub memo: Option<&'r str>,
        pub file: rocket::fs::TempFile<'r>,
    }

    impl<'r> From<FileUploadForm<'r>> for FileUpload {
        fn from(form: FileUploadForm<'r>) -> Self {
            // Note: You'll need to handle the actual file processing separately
            Self {
                account: form.account.to_string(),
                memo: form.memo.map(|s| s.to_string()),
                file: UploadedFile {
                    contents: vec![], // This should be populated from the TempFile
                    filename: form.file.name().unwrap_or_default().to_string(),
                    content_type: form.file.content_type().map(|ct| ct.to_string()).unwrap_or_default(),
                },
            }
        }
    }

    #[derive(FromForm)]
    pub struct FileQueryForm<'r> {
        pub account: &'r str,
        pub memo: Option<&'r str>,
        #[field(name = "file_id")]
        pub file_id: Option<&'r str>,
        #[field(name = "customer_id")]
        pub customer_id: Option<&'r str>,
    }

    impl<'r> From<FileQueryForm<'r>> for FileQuery {
        fn from(form: FileQueryForm<'r>) -> Self {
            Self {
                account: form.account.to_string(),
                memo: form.memo.map(|s| s.to_string()),
                file_id: form.file_id.map(|s| s.to_string()),
                customer_id: form.customer_id.map(|s| s.to_string()),
            }
        }
    }
}