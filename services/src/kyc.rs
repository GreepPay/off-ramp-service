use reqwest::Client;
use stellar_base::KeyPair;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::sep10auth::StellarAuth;
use thiserror::Error;
use std::collections::HashMap;
use models::{
    common::establish_connection,
    sep12::{NewSep12Customer, Sep12Customer, NewSep12CustomerFile, Sep12CustomerFile, NewSep12Callback},
    schema::offramp_service::{sep12_customers,sep12_callbacks , sep12_customer_files,sep12_customers::dsl::*},
};
use diesel::{
    prelude::*,
    OptionalExtension, 
    RunQueryDsl,       
    ExpressionMethods, 
};


#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerQuery {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo_type: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]  // Note the rename
    pub customer_type: Option<String>,  // Now using proper field name internally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

impl CustomerRequest {
    pub fn has_binary_fields(&self) -> bool {
        self.fields.has_binary_fields()
    }
}

#[derive(Debug, Serialize)]
pub struct CustomerRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<String>,
    #[serde(flatten)]
    pub fields: CustomerFields,
}
impl CustomerFields {
    pub fn has_binary_fields(&self) -> bool {
        matches!(self, CustomerFields::Mixed { .. })
    }

    pub fn text_fields(&self) -> HashMap<String, String> {
        match self {
            CustomerFields::Text(fields) => fields.clone(),
            CustomerFields::Mixed { text_fields, .. } => text_fields.clone(),
        }
    }

    pub fn binary_fields(&self) -> Option<HashMap<String, BinaryField>> {
        match self {
            CustomerFields::Mixed { binary_fields, .. } => Some(binary_fields.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum CustomerFields {
    Text(HashMap<String, String>),
    Mixed {
        text_fields: HashMap<String, String>,
        binary_fields: HashMap<String, BinaryField>,
    },
}
#[derive(Debug, Serialize, Clone)]
pub struct BinaryField {
    pub contents: Vec<u8>,
    pub filename: String,
    pub content_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    pub description: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub optional: Option<bool>,
    #[serde(default)]
    pub choices: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldStatus {
    pub status: String,
    #[serde(default)]
    pub error: Option<String>,
}

/// Response from verification request
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResponse {
    /// Current customer status after verification
    pub status: String,

    /// Fields that were successfully verified
    #[serde(default)]
    pub verified_fields: Vec<String>,

    /// Optional human-readable message
    #[serde(default)]
    pub message: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub id: String,

    #[serde(flatten)]
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub fields: Option<HashMap<String, FieldInfo>>,
    #[serde(default)]
    pub provided_fields: Option<HashMap<String, FieldStatus>>,
    #[serde(default)]
    pub message: Option<String>,
}



#[derive(Error, Debug)]
pub enum Sep12Error {
    #[error("Authentication failed: {0}")]
    AuthFailed(#[from] crate::sep10auth::AuthError),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("KYC operation failed: {0}")]
    OperationFailed(String),

    #[error("File operation failed: {0}")]
    FileError(String),

    #[error("Callback failed: {0}")]
    CallbackFailed(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Multipart error: {0}")]
    MultipartError(String),

    #[error("MIME type error: {0}")]
    MimeError(String),

    #[error("Diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),

    #[error("Timestamp verification failed")]
    TimestampError,

    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<serde_json::Error> for Sep12Error {
    fn from(e: serde_json::Error) -> Self {
        Sep12Error::SerializationError(e.to_string())
    }
}

impl From<base64::DecodeError> for Sep12Error {
    fn from(e: base64::DecodeError) -> Self {
        Sep12Error::SerializationError(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Sep12Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Sep12Error::SerializationError(e.to_string())
    }
}


#[derive(Debug, Serialize)]
pub struct CallbackRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo_type: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct CallbackResponse {
    pub success: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FileUpload {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    pub file: UploadedFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedFile {
    pub contents: Vec<u8>,
    pub filename: String,
    pub content_type: String,
}

#[derive(Debug, Serialize)]
pub struct FileQuery {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileResponse {
    pub id: String,
    pub name: String,

}

#[derive(Debug, Deserialize)]
pub struct FileListResponse {
    pub files: Vec<FileResponse>,
}



#[derive(Debug)]
pub struct Sep12Service {
    auth: StellarAuth,
    kyc_server: String,
    http_client: Client,
    keypair: KeyPair,  
}



impl Sep12Service {
    pub fn new(auth: StellarAuth,keypair: KeyPair, kyc_server: String) -> Self {
        Self {
            auth,
            kyc_server,
            http_client: Client::new(),
            keypair,
        }
    }

    // GET /customer 

    pub async fn get_customer(&self, query: CustomerQuery) -> Result<CustomerResponse, Sep12Error> {
        let mut conn = establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

        // First try to get from local database
        let local_customer: Option<Sep12Customer> = sep12_customers
            .filter(account.eq(&query.account))
            .first(&mut conn)
            .optional()
            .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

        if let Some(customer) = local_customer {
            return Ok(CustomerResponse {
                id: customer.id.to_string(),
                status: customer.status,
                fields: None,
                provided_fields: None,
                message: None,
            });
        }

        // Fall back to remote KYC server if not found locally
        let jwt = self.auth.authenticate(&query.account, &self.keypair).await?;

        let mut request = self.http_client
            .get(&format!("{}/customer", self.kyc_server))
            .header("Authorization", format!("Bearer {}", jwt));

        // Add all possible query parameters - properly handle Option<String> to &str conversion
        if let Some(ref  customer_id) = query.id {
            request = request.query(&[("id", customer_id.as_str())]);
        }
        if let Some(ref c_type) = query.customer_type {
            request = request.query(&[("type", c_type.as_str())]);
        }
        if let Some(ref tx_id) = query.transaction_id {
            request = request.query(&[("transaction_id", tx_id.as_str())]);
        }
        if let Some(ref lang) = query.lang {
            request = request.query(&[("lang", lang.as_str())]);
        }

        let response = request.send().await?;
        let customer_response: CustomerResponse = self.handle_response(response).await?;

        // Store the customer in our database
        let new_customer = NewSep12Customer {
            account: query.account,
            memo: query.memo,
            memo_type: query.memo_type,
            customer_type: query.customer_type.unwrap_or_else(|| "individual".to_string()),
            status: customer_response.status.clone(),
            first_name: None,
            last_name: None,
            email: None,
            phone: None,
            date_of_birth: None,
            address_street: None,
            address_city: None,
            address_state: None,
            address_postal_code: None,
            address_country: None,
        };

        diesel::insert_into(sep12_customers)
            .values(&new_customer)
            .execute(&mut conn)
            .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

        Ok(customer_response)
    }
    // PUT /customer - With multipart support
    pub async fn put_customer(&self, request: CustomerRequest) -> Result<CustomerResponse, Sep12Error> {
         let mut conn = establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
         let jwt = self.auth.authenticate(&request.account, &self.keypair).await?;

         // Prepare customer data for database
         let text_fields = request.fields.text_fields();
         let new_customer = NewSep12Customer {
             account: request.account.clone(),
             memo: request.memo.clone(),
             memo_type: request.memo_type.clone(),
             customer_type: request.customer_type.clone().unwrap_or_else(|| "individual".to_string()),
             status: "pending".to_string(), // Default status
             first_name: text_fields.get("first_name").map(|s| s.to_string()),
             last_name: text_fields.get("last_name").map(|s| s.to_string()),
             email: text_fields.get("email").map(|s| s.to_string()),
             phone: text_fields.get("phone").map(|s| s.to_string()),
             date_of_birth: text_fields.get("date_of_birth")
                 .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
             address_street: text_fields.get("address_street").map(|s| s.to_string()),
             address_city: text_fields.get("address_city").map(|s| s.to_string()),
             address_state: text_fields.get("address_state").map(|s| s.to_string()),
             address_postal_code: text_fields.get("address_postal_code").map(|s| s.to_string()),
             address_country: text_fields.get("address_country").map(|s| s.to_string()),
         };

         // Insert or update customer in database
         let customer: Sep12Customer = diesel::insert_into(sep12_customers::table)
             .values(&new_customer)
             .on_conflict(sep12_customers::account)
             .do_update()
             .set(&new_customer)
             .get_result(&mut conn)
             .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

         // Handle file uploads if present
         if let Some(binary_fields) = request.fields.binary_fields() {
             for (field_name, binary_field) in binary_fields {
                 let new_file = NewSep12CustomerFile {
                     customer_id: customer.id,
                     file_name: binary_field.filename.clone(),
                     content_type: binary_field.content_type.clone(),
                     size: binary_field.contents.len() as i64,
                     storage_path: format!("customers/{}/{}", customer.id, Uuid::new_v4()), // Implement your storage logic
                     purpose: field_name,
                 };

                 diesel::insert_into(sep12_customer_files::table)
                     .values(&new_file)
                     .execute(&mut conn)
                     .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
             }
         }

         // Forward to KYC server if needed
         if request.has_binary_fields() {
             let mut form = reqwest::multipart::Form::new();

             for (key, value) in text_fields {
                 form = form.text(key, value);
             }

             if let Some(binary_fields) = request.fields.binary_fields() {
                 for (key, field) in binary_fields {
                     let part = reqwest::multipart::Part::bytes(field.contents)
                         .file_name(field.filename)
                         .mime_str(&field.content_type)?;
                     form = form.part(key, part);
                 }
             }

             let mut req = self.http_client
                 .put(&format!("{}/customer", self.kyc_server))
                 .header("Authorization", format!("Bearer {}", jwt))
                 .multipart(form);

             if let Some(t) = &request.customer_type {
                 req = req.query(&[("type", t)]);
             }

             let response = req.send().await?;
             return self.handle_response(response).await;
         }

         // Standard JSON request
         let mut req = self.http_client
             .put(&format!("{}/customer", self.kyc_server))
             .header("Authorization", format!("Bearer {}", jwt))
             .json(&request);

         if let Some(t) = &request.customer_type {
             req = req.query(&[("type", t)]);
         }

         let response = req.send().await?;
         let customer_response: CustomerResponse = self.handle_response(response).await?;

         // Update status in database
         diesel::update(sep12_customers::table.find(customer.id))
             .set(sep12_customers::status.eq(&customer_response.status))
             .execute(&mut conn)
             .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

         Ok(customer_response)
     }


    // DELETE /customer/[account]

    pub async fn delete_customer(&self, account_str: &str, memo_str: Option<&str>) -> Result<(), Sep12Error> {
        use diesel::prelude::*;

        let mut conn = establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

        // Start with the table, not a filtered query
         let  delete_query = diesel::delete(sep12_customers)
             .filter(account.eq(account_str));

         delete_query
             .execute(&mut conn)
             .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

        // Proceed with remote KYC deletion
        let jwt = self.auth.authenticate(account_str, &self.keypair).await?;

        let mut request = self.http_client
            .delete(&format!("{}/customer/{}", self.kyc_server, account_str))
            .header("Authorization", format!("Bearer {}", jwt));

        if let Some(memo_val) = memo_str {
            request = request.query(&[("memo", memo_val)]);
        }

        let response = request.send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Sep12Error::OperationFailed(format!("Delete failed with status: {}", response.status())))
        }
    }

    // Additional endpoint implementations...
    // PUT /customer/callback
    pub async fn set_callback(
         &self,
         request: CallbackRequest,
     ) -> Result<CallbackResponse, Sep12Error> {
         let mut conn = establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
         let jwt = self.auth.authenticate(&request.account, &self.keypair).await?;

         // Store callback in database
         let new_callback = NewSep12Callback {
             account: request.account.clone(),
             url: request.url.clone(),
         };

         diesel::insert_into(sep12_callbacks::table)
             .values(&new_callback)
             .on_conflict(sep12_callbacks::account)
             .do_update()
             .set(sep12_callbacks::url.eq(&request.url))
             .execute(&mut conn)
             .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

         // Forward to KYC server
         let response = self.http_client
             .put(&format!("{}/customer/callback", self.kyc_server))
             .header("Authorization", format!("Bearer {}", jwt))
             .json(&request)
             .send()
             .await?;

         self.handle_response(response).await
     }

      // Helper for anchors to send callbacks
      pub async fn send_callback(
          &self,
          url: &str,
          data: CustomerResponse,
          signing_key: &KeyPair,
      ) -> Result<(), Sep12Error> {
          let timestamp = Utc::now().timestamp().to_string();
          let body = serde_json::to_vec(&data)?;
          let payload = format!("{}.{}.{}", timestamp, url, String::from_utf8_lossy(&body));

          let signature = signing_key.sign(payload.as_bytes());
          let signature_bytes = signature.as_bytes();  // Correct method to get bytes
          let signature_base64 = BASE64.encode(signature_bytes);

          self.http_client
              .post(url)
              .header("Signature", format!("t={},s={}", timestamp, signature_base64))
              .header("X-Stellar-Signature", format!("t={},s={}", timestamp, signature_base64))
              .json(&data)
              .send()
              .await?;

          Ok(())
      }


    async fn handle_response<T: for<'de> Deserialize<'de>>(&self, response: reqwest::Response) -> Result<T, Sep12Error> {
        if !response.status().is_success() {
            let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".into());
            return Err(Sep12Error::OperationFailed(error_msg));
        }
        response.json().await.map_err(Into::into)
    }


    pub async fn submit_verification(
            &self,
            request: VerificationRequest,
        ) -> Result<CustomerResponse, Sep12Error> {
            // Get the account ID as a string directly from the keypair
            let account_id = self.keypair.public_key().to_string();

               // Authenticate using the string account ID
            let jwt = self.auth.authenticate(&account_id, &self.keypair).await?;

            let response = self.http_client
                .put(&format!("{}/customer/verification", self.kyc_server))
                .header("Authorization", format!("Bearer {}", jwt))
                .json(&request)
                .send()
                .await?;

            self.handle_response(response).await
        }

        pub async fn upload_file(&self, upload: FileUpload) -> Result<FileResponse, Sep12Error> {
            let mut conn = establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
            let jwt = self.auth.authenticate(&upload.account, &self.keypair).await?;

            // First find the customer
            let customer: Sep12Customer = sep12_customers::table
                .filter(sep12_customers::account.eq(&upload.account))
                .first(&mut conn)
                .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

            // Store file metadata in database
            let new_file = NewSep12CustomerFile {
                customer_id: customer.id,
                file_name: upload.file.filename.clone(),
                content_type: upload.file.content_type.clone(),
                size: upload.file.contents.len() as i64,
                storage_path: format!("customers/{}/{}", customer.id, Uuid::new_v4()),
                purpose: "verification".to_string(), // Adjust based on your needs
            };

            let _file: Sep12CustomerFile = diesel::insert_into(sep12_customer_files::table)
                .values(&new_file)
                .get_result(&mut conn)
                .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

            // Then upload to KYC server
            let part = reqwest::multipart::Part::bytes(upload.file.contents)
                .file_name(upload.file.filename)
                .mime_str(&upload.file.content_type)
                .map_err(|e| Sep12Error::MimeError(e.to_string()))?;

            let form = reqwest::multipart::Form::new()
                .part("file", part);

            let response = self.http_client
                .post(&format!("{}/customer/files", self.kyc_server))
                .header("Authorization", format!("Bearer {}", jwt))
                .multipart(form)
                .send()
                .await?;

            self.handle_response(response).await
        }

        pub async fn list_files(&self, query: FileQuery) -> Result<FileListResponse, Sep12Error> {
            let mut conn = establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
            let jwt = self.auth.authenticate(&query.account, &self.keypair).await?;

            // First get from local database
            let customer: Sep12Customer = sep12_customers::table
                .filter(sep12_customers::account.eq(&query.account))
                .first(&mut conn)
                .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

            let mut files_query = sep12_customer_files::table
                .filter(sep12_customer_files::customer_id.eq(customer.id))
                .into_boxed();

            if let Some(file_id) = &query.file_id {  // Take reference here
                if let Ok(uuid) = Uuid::parse_str(file_id) {  // Use the reference directly
                    files_query = files_query.filter(sep12_customer_files::id.eq(uuid));
                }
            }

            let local_files: Vec<Sep12CustomerFile> = files_query
                .load(&mut conn)
                .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

            if !local_files.is_empty() {
                return Ok(FileListResponse {
                    files: local_files.into_iter().map(|f| FileResponse {
                        id: f.id.to_string(),  // Convert to String
                        name: f.file_name.clone(),  // Clone the String
                    }).collect(),
                });
            }

            // Fall back to remote KYC server if no local files
            let mut request = self.http_client
                .get(&format!("{}/customer/files", self.kyc_server))
                .header("Authorization", format!("Bearer {}", jwt));

            if let Some(file_id) = &query.file_id {  // Take reference here
                request = request.query(&[("file_id", file_id.as_str())]);
            }
            if let Some(customer_id) = &query.customer_id {  // Take reference here
                request = request.query(&[("customer_id", customer_id.as_str())]);
            }

            let response = request.send().await?;
            self.handle_response(response).await
        }
}