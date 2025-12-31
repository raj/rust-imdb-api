//! Rust IMDB API client
//!
//! This crate provides a Rust interface to the IMDB JSON web service,
//! which is the same API used by the IMDB iOS app.
//!
//! # Example
//!
//! ```no_run
//! use imdb_api::ImdbClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ImdbClient::new();
//!
//!     // Get title information
//!     let title = client.get_title("tt0111161").await?;
//!     println!("{:?}", title);
//!
//!     // Search for titles
//!     let results = client.search("The Dark Knight").await?;
//!     println!("{:?}", results);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod constants;
pub mod error;
pub mod signer;

pub use client::{validate_imdb_id, ImdbClient};
pub use error::{ImdbApiError, Result};
pub use signer::{AuthHeaders, Credentials};

/// Validate an IMDB ID (re-exported from client module)
pub fn is_valid_imdb_id(imdb_id: &str) -> bool {
    validate_imdb_id(imdb_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_imdb_id() {
        assert!(is_valid_imdb_id("tt0111161"));
        assert!(is_valid_imdb_id("nm0000151"));
        assert!(!is_valid_imdb_id("invalid"));
        assert!(!is_valid_imdb_id("tt123"));
    }
}
