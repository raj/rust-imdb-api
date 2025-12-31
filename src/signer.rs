use crate::error::{ImdbApiError, Result};
use base64::Engine;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

/// AWS3-style authentication credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
}

/// Authentication headers returned by the signer
#[derive(Debug, Clone)]
pub struct AuthHeaders {
    pub x_amz_date: String,
    pub x_amz_security_token: String,
    pub x_amzn_authorization: String,
}

/// AWS3-style request signer
pub struct Signer;

impl Signer {
    /// Sign a request URL with the provided credentials
    pub fn sign(url: &str, creds: &Credentials) -> Result<AuthHeaders> {
        let parsed_url = url::Url::parse(url)
            .map_err(|e| ImdbApiError::ApiError(format!("Invalid URL: {}", e)))?;

        let headers = Self::get_auth_headers(&parsed_url, creds)?;
        Ok(headers)
    }

    fn get_auth_headers(url: &url::Url, creds: &Credentials) -> Result<AuthHeaders> {
        let now: DateTime<Utc> = Utc::now();
        let amz_date = now.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

        let string_to_sign = format!(
            "GET\n{}\n\nhost:api.imdbws.com\nx-amz-date:{}\nx-amz-security-token:{}\n\n",
            url.path(),
            amz_date,
            creds.session_token
        );

        // Compute SHA-256 hash of the string to sign
        let hash = sha2::Sha256::digest(string_to_sign.as_bytes());

        // Compute HMAC-SHA256
        let mut mac =
            HmacSha256::new_from_slice(creds.secret_access_key.as_bytes())
                .map_err(|_| ImdbApiError::AuthenticationError)?;
        mac.update(&hash);
        let hmac_result = mac.finalize().into_bytes();

        // Base64 encode the HMAC result
        let b64_hmac = base64::engine::general_purpose::STANDARD.encode(&hmac_result);

        // Build authorization header
        let authorization = format!(
            "AWS3 AWSAccessKeyId={},Algorithm=HmacSHA256,SignedHeaders=Host;X-Amz-Date;X-Amz-Security-Token,Signature={}",
            creds.access_key_id, b64_hmac
        );

        Ok(AuthHeaders {
            x_amz_date: amz_date,
            x_amz_security_token: creds.session_token.clone(),
            x_amzn_authorization: authorization,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signer_creates_headers() {
        let creds = Credentials {
            access_key_id: "test_key".to_string(),
            secret_access_key: "test_secret".to_string(),
            session_token: "test_token".to_string(),
        };

        let result = Signer::sign("https://api.imdbws.com/test", &creds);
        assert!(result.is_ok());

        let headers = result.unwrap();
        assert!(!headers.x_amz_date.is_empty());
        assert!(!headers.x_amz_security_token.is_empty());
        assert!(!headers.x_amzn_authorization.is_empty());
    }
}
