pub mod form {

    use rocket::serde::{Deserialize, Serialize,};
    use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

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
    pub struct Sep12CreateKycForm {
        pub slug: String,
        pub account: String,
        #[serde(default)]
        pub memo: Option<String>,
        pub customer_type: String,
    }
    
    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep12UpdateKycForm {
        pub slug: String,
        pub customer_id: String,
    }
    
    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep12DeleteKycForm {
        pub slug: String,
        pub account: String,
        #[serde(default)]
        pub memo: Option<String>,
    }
    

    #[derive(SerdeDeserialize, SerdeSerialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep12FileField {
        pub name: String,
        #[serde(rename = "file")]
        pub data: Vec<u8>,  // For binary file data in JSON (base64 encoded)
        #[serde(rename = "content_type")]
        pub content_type: String,
    }
    
    #[derive(SerdeDeserialize, SerdeSerialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep12FieldsAndFiles {
        #[serde(rename = "field")]
        pub fields: Vec<(String, String)>,
        #[serde(rename = "field")]
        pub files: Vec<Sep12FileField>,
    }
    
}