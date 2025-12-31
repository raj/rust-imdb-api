use crate::constants::*;
use crate::error::{ImdbApiError, Result};
use crate::signer::{Credentials, Signer};
use regex::Regex;
use reqwest::Client as HttpClient;
use serde_json::Value;
use std::collections::HashMap;

/// IMDB API client
#[derive(Debug, Clone)]
pub struct ImdbClient {
    http_client: HttpClient,
}

impl ImdbClient {
    /// Create a new IMDB API client
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
        }
    }

    /// Check if a title exists on IMDB
    pub async fn title_exists(&self, imdb_id: &str) -> Result<bool> {
        if !validate_imdb_id(imdb_id) {
            return Ok(false);
        }

        let url = format!("https://www.imdb.com/title/{}/", imdb_id);

        let response = self
            .http_client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .send()
            .await?;

        // Accept 200 OK or redirect status codes (301, 302, 303, 307, 308)
        let status = response.status();
        Ok(status.is_success() || status.is_redirection())
    }

    /// Search for titles or names on IMDB
    pub async fn search(&self, query: &str) -> Result<Value> {
        let clean_q = query
            .replace(' ', "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.' || *c == '~')
            .collect::<String>()
            .to_lowercase();

        let first_char = clean_q
            .chars()
            .next()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "a".to_string());

        let search_url = urlencoding::encode(&clean_q);
        let url = format!(
            "{}/suggests/{}/{}.json",
            SEARCH_BASE_URI, first_char, search_url
        );

        let response = self.http_client.get(&url).send().await?;
        let text = response.text().await?;

        // Parse the special IMDB suggest format (imdb$NAME({...}))
        let re = Regex::new(r#"imdb\$\w+\((\{.+\})\)"#).unwrap();
        if let Some(captures) = re.captures(&text) {
            if let Some(json_str) = captures.get(1) {
                return Ok(serde_json::from_str(json_str.as_str())?);
            }
        }

        // Try parsing as regular JSON
        Ok(serde_json::from_str(&text)?)
    }

    /// Get popular titles
    pub async fn get_popular_titles(&self) -> Result<Value> {
        self.get_resource(CHART_TITLEMETER, "").await
    }

    /// Get popular TV shows
    pub async fn get_popular_shows(&self) -> Result<Value> {
        self.get_resource(CHART_TVMETER, "").await
    }

    /// Get popular movies
    pub async fn get_popular_movies(&self) -> Result<Value> {
        self.get_resource(CHART_MOVIMETER, "").await
    }

    /// Get title information
    pub async fn get_title(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/auxiliary", imdb_id)
            .await
    }

    /// Get name information
    pub async fn get_name(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/name/{imdb_id}/fulldetails", imdb_id)
            .await
    }

    /// Get name filmography
    pub async fn get_name_filmography(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/name/{imdb_id}/filmography", imdb_id)
            .await
    }

    /// Get name images
    pub async fn get_name_images(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/name/{imdb_id}/images", imdb_id)
            .await
    }

    /// Get name videos
    pub async fn get_name_videos(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/name/{imdb_id}/videos", imdb_id)
            .await
    }

    /// Get title genres
    pub async fn get_title_genres(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/genres", imdb_id)
            .await
    }

    /// Get title credits
    pub async fn get_title_credits(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/fullcredits", imdb_id)
            .await
    }

    /// Get title quotes
    pub async fn get_title_quotes(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/quotes", imdb_id)
            .await
    }

    /// Get title ratings
    pub async fn get_title_ratings(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/ratings", imdb_id)
            .await
    }

    /// Get title connections
    pub async fn get_title_connections(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/connections", imdb_id)
            .await
    }

    /// Get title similarities
    pub async fn get_title_similarities(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/similarities", imdb_id)
            .await
    }

    /// Get title videos
    pub async fn get_title_videos(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/videos", imdb_id)
            .await
    }

    /// Get title news
    pub async fn get_title_news(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/news", imdb_id)
            .await
    }

    /// Get title trivia
    pub async fn get_title_trivia(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/trivia", imdb_id)
            .await
    }

    /// Get title soundtracks
    pub async fn get_title_soundtracks(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/soundtracks", imdb_id)
            .await
    }

    /// Get title goofs
    pub async fn get_title_goofs(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/goofs", imdb_id)
            .await
    }

    /// Get title technical information
    pub async fn get_title_technical(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/technical", imdb_id)
            .await
    }

    /// Get title companies
    pub async fn get_title_companies(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/companies", imdb_id)
            .await
    }

    /// Get title episodes (TV shows)
    pub async fn get_title_episodes(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/episodes", imdb_id)
            .await
    }

    /// Get title plot
    pub async fn get_title_plot(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/plot", imdb_id)
            .await
    }

    /// Get title plot synopsis
    pub async fn get_title_plot_synopsis(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/plotsynopsis", imdb_id)
            .await
    }

    /// Get title awards
    pub async fn get_title_awards(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/awards", imdb_id)
            .await
    }

    /// Get title releases
    pub async fn get_title_releases(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/releases", imdb_id)
            .await
    }

    /// Get title versions
    pub async fn get_title_versions(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/versions", imdb_id)
            .await
    }

    /// Get title user reviews
    pub async fn get_title_user_reviews(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/userreviews", imdb_id)
            .await
    }

    /// Get title metacritic reviews
    pub async fn get_title_metacritic_reviews(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/metacritic", imdb_id)
            .await
    }

    /// Get title images
    pub async fn get_title_images(&self, imdb_id: &str) -> Result<Value> {
        self.get_resource("/title/{imdb_id}/images", imdb_id)
            .await
    }

    /// Internal method to get a resource from the API
    async fn get_resource(&self, endpoint: &str, imdb_id: &str) -> Result<Value> {
        let path = endpoint.replace("{imdb_id}", imdb_id);
        let url = format!("{}{}", BASE_URI, path);

        let creds = self.get_credentials().await?;
        let auth_headers = Signer::sign(&url, &creds)?;

        let response = self
            .http_client
            .get(&url)
            .header("content-type", "application/json")
            .header("accept-language", "en_US")
            .header("x-amz-date", auth_headers.x_amz_date)
            .header("x-amz-security-token", auth_headers.x_amz_security_token)
            .header("x-amzn-authorization", auth_headers.x_amzn_authorization)
            .header("user-agent", USER_AGENT)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ImdbApiError::ApiError(format!(
                "API returned status: {}",
                response.status()
            )));
        }

        let json: Value = response.json().await?;
        Ok(json["resource"].clone())
    }

    /// Get temporary credentials for API access
    async fn get_credentials(&self) -> Result<Credentials> {
        let url = format!("{}/authentication/credentials/temporary/ios82?=", BASE_URI);

        let mut body = HashMap::new();
        body.insert("appKey", APP_KEY);

        let response = self
            .http_client
            .post(&url)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ImdbApiError::AuthenticationError);
        }

        let json: Value = response.json().await?;

        let resource = json["resource"]
            .as_object()
            .ok_or(ImdbApiError::AuthenticationError)?;

        Ok(Credentials {
            access_key_id: resource["accessKeyId"]
                .as_str()
                .ok_or(ImdbApiError::AuthenticationError)?
                .to_string(),
            secret_access_key: resource["secretAccessKey"]
                .as_str()
                .ok_or(ImdbApiError::AuthenticationError)?
                .to_string(),
            session_token: resource["sessionToken"]
                .as_str()
                .ok_or(ImdbApiError::AuthenticationError)?
                .to_string(),
        })
    }
}

impl Default for ImdbClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate an IMDB ID
pub fn validate_imdb_id(imdb_id: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z]{2}[0-9]{7}$").unwrap();
    re.is_match(imdb_id) && imdb_id.len() == 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_imdb_id() {
        assert!(validate_imdb_id("tt0111161"));
        assert!(validate_imdb_id("nm0000151"));
        assert!(!validate_imdb_id("invalid"));
        assert!(!validate_imdb_id("tt123"));
    }
}
