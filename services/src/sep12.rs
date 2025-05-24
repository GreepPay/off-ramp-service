pub mod sep12 {

    use crate::common::get_anchor_config_details;
    use diesel::prelude::*;
    use helpers::{auth::authenticate, keypair::generate_keypair};
    use rocket::fs::TempFile;
    use rocket::tokio::io::AsyncReadExt;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use models::{
        common::establish_connection,
        schema::offramp_service::{sep12_customer_files, sep12_customers},
        sep12::{NewSep12Customer, NewSep12CustomerFile, Sep12Customer},
    };

    #[derive(Error, Debug)]
    pub enum Sep12Error {
        #[error("HTTP error: {0}")]
        HttpError(#[from] reqwest::Error),

        #[error("Authentication failed")]
        AuthFailed, // Will be updated later

        #[error("Keypair failed")]
        Keypairgenerationfailed, // Will be updated later

        #[error("Invalid request: {0}")]
        InvalidRequest(String),

        #[error("Customer not found")]
        CustomerNotFound,

        #[error("Verification failed: {0}")]
        VerificationFailed(String),

        #[error("Database error: {0}")]
        DatabaseError(String),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Customer {
        pub id: Option<String>,
        pub account: Option<String>,
        pub memo: Option<String>,
        #[serde(rename = "type")]
        pub customer_type: Option<String>,
        pub status: String,
        pub fields: Option<Fields>,
        pub provided_fields: Option<Vec<ProvidedField>>,
        pub message: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Fields {
        pub first_name: Option<Field>,
        pub last_name: Option<Field>,
        pub email_address: Option<Field>,
        pub mobile_number: Option<Field>,
        pub bank_account_number: Option<Field>,
        pub photo_id_front: Option<Field>,
        pub photo_proof_residence: Option<Field>,
        pub proof_of_liveness: Option<Field>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Field {
        #[serde(rename = "type")]
        pub field_type: String,
        pub description: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProvidedField {
        pub name: String,
        pub description: String,
        #[serde(rename = "type")]
        pub field_type: String,
        pub status: String,
        pub error: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct VerificationRequest {
        pub id: String,
        pub field: String,
        pub value: String,
    }

    // 1. Get required verification fields
    pub async fn get_required_verification(
        slug: &str,
        account: &str,
        memo: Option<&str>,
        customer_type: Option<&str>,
    ) -> Result<Fields, Sep12Error> {
        let client = reqwest::Client::new();

        let keypair = match generate_keypair(account) {
            Ok(kp) => kp,
            Err(_) => {
                return Err(Sep12Error::Keypairgenerationfailed);
            }
        };

        // Get anchor configuration for authentication
        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
                .await
                .map_err(|_| Sep12Error::AuthFailed)?;

        let jwt =
            match authenticate(&helpers::stellartoml::AnchorService::new(), slug, &keypair).await {
                Ok(jwt) => jwt,
                Err(e) => {
                    println!("Authentication error: {}", e);
                    return Err(Sep12Error::AuthFailed);
                }
            };

        let kyc_server = &anchor_config.general_info.kyc_server;
        // Unwrap the Option or provide a default value
        let kyc_server_str = kyc_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );

        let mut request = client
            .get(&format!("{}/customer", kyc_server_str))
            .bearer_auth(jwt);

        if let Some(m) = memo {
            request = request.query(&[("memo", m)]);
        }

        if let Some(t) = customer_type {
            request = request.query(&[("type", t)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let customer: Customer = response.json().await?;
            Ok(customer.fields.unwrap())
        } else if response.status() == 404 {
            Err(Sep12Error::CustomerNotFound)
        } else {
            Err(Sep12Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }

    // 2. Get account KYC status
    pub async fn get_account_kyc(
        slug: &str,
        account: &str,
        memo: Option<&str>,
        customer_type: Option<&str>,
    ) -> Result<Customer, Sep12Error> {
        let client = reqwest::Client::new();

        let keypair = match generate_keypair(account) {
            Ok(kp) => kp,
            Err(_) => {
                println!("Keypair generation failed");
                return Err(Sep12Error::Keypairgenerationfailed);
            }
        };
        // Get anchor configuration for authentication
        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
                .await
                .map_err(|_| {
                    println!("Authentication failed in get_account_kyc");
                    Sep12Error::AuthFailed
                })?;

        let jwt =
            match authenticate(&helpers::stellartoml::AnchorService::new(), slug, &keypair).await {
                Ok(token) => token,
                Err(_) => {
                    println!("JWT authentication failed");
                    return Err(Sep12Error::Keypairgenerationfailed);
                }
            };

        let kyc_server = &anchor_config.general_info.kyc_server;
        // Unwrap the Option or provide a default value
        let kyc_server_str = kyc_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );
        let mut request = client
            .get(&format!("{}/customer", kyc_server_str))
            .bearer_auth(jwt);

        if let Some(m) = memo {
            request = request.query(&[("memo", m)]);
            println!("Adding memo to the request: {}", m);
        }

        if let Some(t) = customer_type {
            request = request.query(&[("type", t)]);
            println!("Adding customer_type to the request: {}", t);
        }

        let response = request.send().await?;
        println!("Response status: {}", response.status());

        if response.status().is_success() {
            let customer: Customer = response.json().await?;

            Ok(customer)
        } else if response.status() == 404 {
            println!("Customer not found");
            Err(Sep12Error::CustomerNotFound)
        } else {
            let status = response.status();
            println!("Invalid request with status: {}", status);
            Err(Sep12Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }

    // 3. Create account KYC
    pub async fn create_account_kyc<'v>(
        slug: &str,
        account: &str,
        memo: Option<&str>,
        customer_type: &str,
        fields: Vec<(String, String)>,
        files: Vec<(String, TempFile<'v>)>,
    ) -> Result<Customer, Sep12Error> {
        let client = reqwest::Client::new();

        let keypair = match generate_keypair(account) {
            Ok(kp) => kp,
            Err(_) => return Err(Sep12Error::AuthFailed),
        };
        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
                .await
                .map_err(|_| Sep12Error::AuthFailed)?;

        let jwt =
            match authenticate(&helpers::stellartoml::AnchorService::new(), slug, &keypair).await {
                Ok(token) => token,
                Err(_) => return Err(Sep12Error::Keypairgenerationfailed),
            };

        let kyc_server = &anchor_config.general_info.kyc_server;
        // Unwrap the Option or provide a default value
        let kyc_server_str = kyc_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );
        let mut form = reqwest::multipart::Form::new()
            .text("account", keypair.public_key().to_string())
            .text("type", customer_type.to_string());

        if let Some(m) = memo {
            form = form.text("memo", m.to_string());
        }

        form = form.text("memo_type", "id".to_string());

        for (name, value) in &fields {
            form = form.text(name.to_string(), value.to_string());
        }

        // Add files
        for (name, temp_file) in &files {
            let file_name = temp_file
                .name()
                .map(|n| n.to_string())
                .unwrap_or_else(|| name.clone());

            let content_type = temp_file
                .content_type()
                .map(|ct| ct.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());

            let mut buf = Vec::new();
            let mut file = temp_file
                .open()
                .await
                .map_err(|e| Sep12Error::InvalidRequest(format!("Failed to open file: {}", e)))?;

            file.read_to_end(&mut buf)
                .await
                .map_err(|e| Sep12Error::InvalidRequest(format!("Failed to read file: {}", e)))?;

            let part = reqwest::multipart::Part::bytes(buf)
                .file_name(file_name)
                .mime_str(&content_type)
                .map_err(|e| Sep12Error::InvalidRequest(format!("Invalid MIME type: {}", e)))?;

            form = form.part(name.to_string(), part);
        }

        let response = client
            .put(&format!("{}/customer", kyc_server_str))
            .bearer_auth(jwt)
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let customer_response: Customer = response.json().await?;

            // Save to database
            let mut conn =
                establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

            let new_customer = NewSep12Customer {
                account: account.to_string(),
                memo: memo.map(|s| s.to_string()),
                memo_type: Some("id".to_string()), // You may need to get this from somewhere
                customer_type: customer_type.to_string(),
                status: customer_response.status.clone(),
                first_name: fields
                    .iter()
                    .find(|(n, _)| *n == "first_name")
                    .map(|(_, v)| v.to_string()),
                last_name: fields
                    .iter()
                    .find(|(n, _)| *n == "last_name")
                    .map(|(_, v)| (*v).to_string()),
                email: fields
                    .iter()
                    .find(|(n, _)| *n == "email")
                    .map(|(_, v)| (*v).to_string()),
                phone: fields
                    .iter()
                    .find(|(n, _)| *n == "phone_number")
                    .map(|(_, v)| (*v).to_string()),
                date_of_birth: None, // Parse from fields if available
                address_street: None,
                address_city: None,
                address_state: None,
                address_postal_code: None,
                address_country: None,
            };

            let customer: Sep12Customer = diesel::insert_into(sep12_customers::table)
                .values(&new_customer)
                .get_result(&mut conn)
                .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

            // Save files if any
            for (name, temp_file) in &files {
                let file_id = Uuid::new_v4();
                let storage_path = format!("customers/{}/{}_{}", customer.id, file_id, name);

                let file_name = temp_file
                    .name()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| name.clone());
                let content_type = temp_file
                    .content_type()
                    .map(|ct| ct.to_string())
                    .unwrap_or_else(|| "application/octet-stream".to_string());

                let mut file_content = Vec::new();
                let mut file = temp_file.open().await.map_err(|e| {
                    Sep12Error::InvalidRequest(format!("Failed to open file: {}", e))
                })?;

                file.read_to_end(&mut file_content).await.map_err(|e| {
                    Sep12Error::InvalidRequest(format!("Failed to read file: {}", e))
                })?;

                let new_file = NewSep12CustomerFile {
                    customer_id: customer.id,
                    file_name: file_name.to_string(),
                    content_type: content_type.to_string(),
                    size: file_content.len() as i64,
                    storage_path: storage_path.clone(),
                    purpose: name.to_string(),
                };

                diesel::insert_into(sep12_customer_files::table)
                    .values(&new_file)
                    .execute(&mut conn)
                    .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
            }

            Ok(customer_response)
        } else {
            let error = response.text().await?;
            Err(Sep12Error::InvalidRequest(error))
        }
    }

    // 4. Update account KYC
    pub async fn update_account_kyc(
        slug: &str,
        customer_id: &str,
        fields: Vec<(String, String)>,
        files: Vec<(String, Vec<u8>, String)>,
        account: &str,
    ) -> Result<Customer, Sep12Error> {
        let client = reqwest::Client::new();

        let keypair = match generate_keypair(account) {
            Ok(kp) => kp,
            Err(_) => return Err(Sep12Error::AuthFailed),
        };
        let mut conn =
            establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
        // Get customer from database
        let customer_uuid = Uuid::parse_str(customer_id)
            .map_err(|_| Sep12Error::InvalidRequest("Invalid customer ID".to_string()))?;

        let customer: Sep12Customer = sep12_customers::table
            .find(customer_uuid)
            .first(&mut conn)
            .map_err(|_| Sep12Error::CustomerNotFound)?;

        // Get anchor configuration for authentication
        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
                .await
                .map_err(|_| Sep12Error::AuthFailed)?;

        let jwt =
            match authenticate(&helpers::stellartoml::AnchorService::new(), slug, &keypair).await {
                Ok(token) => token,
                Err(_) => return Err(Sep12Error::Keypairgenerationfailed),
            };

        let kyc_server = &anchor_config.general_info.kyc_server;
        // Unwrap the Option or provide a default value
        let kyc_server_str = kyc_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );
        let mut form = reqwest::multipart::Form::new().text("id", customer_id.to_string());

        for (name, value) in fields.iter() {
            form = form.text(name.to_string(), value.to_string());
        }

        for (name, content, content_type) in files.iter() {
            let part = reqwest::multipart::Part::bytes(content.clone())
                .file_name(name.to_string())
                .mime_str(content_type)?;
            form = form.part(name.to_string(), part);
        }

        let response = client
            .put(&format!("{}/customer", kyc_server_str))
            .bearer_auth(jwt)
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let customer_response: Customer = response.json().await?;

            // Update customer in database
            let first_name = fields
                .iter()
                .find(|(n, _)| *n == "first_name")
                .map(|(_, v)| v.to_string());
            let last_name = fields
                .iter()
                .find(|(n, _)| *n == "last_name")
                .map(|(_, v)| v.to_string());
            let email = fields
                .iter()
                .find(|(n, _)| *n == "email")
                .map(|(_, v)| v.to_string());
            let phone = fields
                .iter()
                .find(|(n, _)| *n == "phone_number")
                .map(|(_, v)| v.to_string());

            diesel::update(sep12_customers::table.find(customer.id))
                .set((
                    sep12_customers::status.eq(&customer_response.status),
                    sep12_customers::first_name.eq(first_name.as_ref()),
                    sep12_customers::last_name.eq(last_name.as_ref()),
                    sep12_customers::email.eq(email.as_ref()),
                    sep12_customers::phone.eq(phone.as_ref()),
                ))
                .execute(&mut conn)
                .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

            // Save new files if any
            for (name, content, content_type) in files {
                let file_id = Uuid::new_v4();
                let storage_path = format!("customers/{}/{}_{}", customer.id, file_id, name);

                // In a real implementation, you would save the file to storage here

                let new_file = NewSep12CustomerFile {
                    customer_id: customer.id,
                    file_name: name.to_string(),
                    content_type: content_type.to_string(),
                    size: content.len() as i64,
                    storage_path: storage_path.clone(),
                    purpose: name.to_string(),
                };

                diesel::insert_into(sep12_customer_files::table)
                    .values(&new_file)
                    .execute(&mut conn)
                    .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
            }

            Ok(customer_response)
        } else {
            let error = response.text().await?;
            Err(Sep12Error::InvalidRequest(error))
        }
    }

    // 5. Delete account KYC
    pub async fn delete_account_kyc(
        slug: &str,
        account: &str,
        memo: Option<&str>,
    ) -> Result<(), Sep12Error> {
        let client = reqwest::Client::new();

        let keypair = match generate_keypair(account) {
            Ok(kp) => kp,
            Err(_) => return Err(Sep12Error::AuthFailed),
        };
        // Get anchor configuration for authentication
        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
                .await
                .map_err(|_| Sep12Error::AuthFailed)?;

        let jwt =
            match authenticate(&helpers::stellartoml::AnchorService::new(), slug, &keypair).await {
                Ok(token) => token,
                Err(_) => return Err(Sep12Error::Keypairgenerationfailed),
            };

        let kyc_server = &anchor_config.general_info.kyc_server;
        // Unwrap the Option or provide a default value
        let kyc_server_str = kyc_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );
        let mut request = client
            .delete(&format!("{}/customer/{}", kyc_server_str, account))
            .bearer_auth(jwt);

        if let Some(m) = memo {
            request = request.query(&[("memo", m)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            // Delete from database
            let mut conn =
                establish_connection().map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;

            match memo {
                Some(m) => {
                    diesel::delete(
                        sep12_customers::table
                            .filter(sep12_customers::account.eq(account))
                            .filter(sep12_customers::memo.eq(m)),
                    )
                    .execute(&mut conn)
                    .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
                }
                __ => {
                    diesel::delete(
                        sep12_customers::table.filter(sep12_customers::account.eq(account)),
                    )
                    .execute(&mut conn)
                    .map_err(|e| Sep12Error::DatabaseError(e.to_string()))?;
                }
            }

            Ok(())
        } else if response.status() == 404 {
            Err(Sep12Error::CustomerNotFound)
        } else {
            Err(Sep12Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }
}
