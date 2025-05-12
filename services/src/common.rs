// Function to extract and format all anchor configuration info
// 
use helpers::stellartoml::{AnchorService, AnchorError};

pub async fn get_anchor_config_details(anchor_service: &AnchorService, slug: &str) -> Result<AnchorConfigDetails, AnchorError> {
    let anchor_config = anchor_service.get_anchor(slug).await?;

    // Convert the config into a structured format for easy access
    let details = AnchorConfigDetails {
        general_info: GeneralInfo {
            version: anchor_config.version.clone(),
            network_passphrase: anchor_config.network_passphrase.clone(),
            federation_server: anchor_config.federation_server.clone(),
            auth_server: anchor_config.auth_server.clone(),
            transfer_server: anchor_config.transfer_server.clone(),
            transfer_server_sep0024: anchor_config.transfer_server_sep0024.clone(),
            kyc_server: anchor_config.kyc_server.clone(),
            web_auth_endpoint: anchor_config.web_auth_endpoint.clone(),
            web_auth_for_contracts_endpoint: anchor_config.web_auth_for_contracts_endpoint.clone(),
            web_auth_contract_id: anchor_config.web_auth_contract_id.clone(),
            signing_key: anchor_config.signing_key.clone(),
            horizon_url: anchor_config.horizon_url.clone(),
            accounts: anchor_config.accounts.clone(),
            uri_request_signing_key: anchor_config.uri_request_signing_key.clone(),
            direct_payment_server: anchor_config.direct_payment_server.clone(),
            anchor_quote_server: anchor_config.anchor_quote_server.clone(),
        },
        documentation: match anchor_config.documentation {
            Some(doc) => Some(DocumentationInfo {
                org_name: doc.org_name.clone(),
                org_dba: doc.org_dba.clone(),
                org_url: doc.org_url.clone(),
                org_logo: doc.org_logo.clone(),
                org_description: doc.org_description.clone(),
                org_physical_address: doc.org_physical_address.clone(),
                org_physical_address_attestation: doc.org_physical_address_attestation.clone(),
                org_phone_number: doc.org_phone_number.clone(),
                org_phone_number_attestation: doc.org_phone_number_attestation.clone(),
                org_keybase: doc.org_keybase.clone(),
                org_twitter: doc.org_twitter.clone(),
                org_github: doc.org_github.clone(),
                org_official_email: doc.org_official_email.clone(),
                org_support_email: doc.org_support_email.clone(),
                org_licensing_authority: doc.org_licensing_authority.clone(),
                org_license_type: doc.org_license_type.clone(),
                org_license_number: doc.org_license_number.clone(),
            }),
            __=> None,
        },
    };

    Ok(details)
}

// Structured representation of anchor configuration
#[derive(Debug, Clone)]
pub struct AnchorConfigDetails {
    pub general_info: GeneralInfo,
    pub documentation: Option<DocumentationInfo>,
}

#[derive(Debug, Clone)]
pub struct GeneralInfo {
    pub version: String,
    pub network_passphrase: String,
    pub federation_server: Option<String>,
    pub auth_server: Option<String>,
    pub transfer_server: Option<String>,
    pub transfer_server_sep0024: Option<String>,
    pub kyc_server: Option<String>,
    pub web_auth_endpoint: String,
    pub web_auth_for_contracts_endpoint: Option<String>,
    pub web_auth_contract_id: Option<String>,
    pub signing_key: String,
    pub horizon_url: Option<String>,
    pub accounts: Option<Vec<String>>,
    pub uri_request_signing_key: Option<String>,
    pub direct_payment_server: Option<String>,
    pub anchor_quote_server: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DocumentationInfo {
    pub org_name: Option<String>,
    pub org_dba: Option<String>,
    pub org_url: Option<String>,
    pub org_logo: Option<String>,
    pub org_description: Option<String>,
    pub org_physical_address: Option<String>,
    pub org_physical_address_attestation: Option<String>,
    pub org_phone_number: Option<String>,
    pub org_phone_number_attestation: Option<String>,
    pub org_keybase: Option<String>,
    pub org_twitter: Option<String>,
    pub org_github: Option<String>,
    pub org_official_email: Option<String>,
    pub org_support_email: Option<String>,
    pub org_licensing_authority: Option<String>,
    pub org_license_type: Option<String>,
    pub org_license_number: Option<String>,
}