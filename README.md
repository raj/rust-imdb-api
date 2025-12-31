# Rust IMDB API

Rust IMDB client using the IMDB JSON web service made available for their iOS app.
Inspired by [imdb-apios](https://github.com/raj/imdb-apios) Ruby gem.

## API Terminology

- `Title` - This can be a movie, TV show, video, documentary, etc.
- `Name` - This can be a credit, cast member, or any person.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
imdb-api = "0.1"
```

## Usage

### Basic Example

```rust
use imdb_api::ImdbClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ImdbClient::new();

    // Get title information
    let title = client.get_title("tt0111161").await?;
    println!("{:?}", title);

    // Search for titles
    let results = client.search("The Dark Knight").await?;
    println!("{:?}", results);

    Ok(())
}
```

### Validate IMDB ID

```rust
use imdb_api::validate_imdb_id;

fn main() {
    let is_valid = validate_imdb_id("tt0111161");  // true
    let is_invalid = validate_imdb_id("invalid");  // false
}
```

### Check if Title Exists

```rust
use imdb_api::ImdbClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ImdbClient::new();
    let exists = client.title_exists("tt0111161").await?;
    println!("Title exists: {}", exists);
    Ok(())
}
```

### Available Methods

#### Title Methods

| Method | Description |
|--------|-------------|
| `get_title(id)` | Returns title information |
| `get_title_genres(id)` | Returns title genres information |
| `get_title_credits(id)` | Returns title credits information |
| `get_title_quotes(id)` | Returns title quotes information |
| `get_title_ratings(id)` | Returns title ratings information |
| `get_title_connections(id)` | Returns title connections information |
| `get_title_similarities(id)` | Returns title similarities information |
| `get_title_videos(id)` | Returns title videos information |
| `get_title_news(id)` | Returns news |
| `get_title_trivia(id)` | Returns trivia |
| `get_title_soundtracks(id)` | Returns soundtracks information |
| `get_title_goofs(id)` | Returns "goofs" information |
| `get_title_technical(id)` | Returns technical information |
| `get_title_companies(id)` | Returns information about companies |
| `get_title_episodes(id)` | Returns season and episodes information |
| `get_title_plot(id)` | Returns title plot information |
| `get_title_plot_synopsis(id)` | Returns title plot synopsis information |
| `get_title_awards(id)` | Returns title awards information |
| `get_title_releases(id)` | Returns releases information |
| `get_title_versions(id)` | Returns versions information |
| `get_title_user_reviews(id)` | Returns user review information |
| `get_title_metacritic_reviews(id)` | Returns metacritic review information |
| `get_title_images(id)` | Returns title images information |

#### Name Methods

| Method | Description |
|--------|-------------|
| `get_name(id)` | Returns person/name information |
| `get_name_filmography(id)` | Returns person/name filmography information |
| `get_name_images(id)` | Returns person/name images information |
| `get_name_videos(id)` | Returns person/name videos information |

#### Search Methods

| Method | Description |
|--------|-------------|
| `search(query)` | Returns search results for titles or names |
| `get_popular_titles()` | Returns popular titles |
| `get_popular_shows()` | Returns popular TV shows |
| `get_popular_movies()` | Returns popular movies |

#### Validation

| Method | Description |
|--------|-------------|
| `validate_imdb_id(id)` | Returns `true` if IMDB ID is valid, `false` otherwise |
| `title_exists(id)` | Returns `true` if title exists on IMDB, `false` otherwise |

## Error Handling

All methods return `Result<T, ImdbApiError>` where `ImdbApiError` can be:

- `RequestError` - HTTP request error
- `JsonError` - JSON parsing error
- `InvalidImdbId` - Invalid IMDB ID format
- `ApiError` - API error
- `TitleNotFound` - Title not found
- `AuthenticationError` - Authentication error

## Example Project

See `/Users/rajdeenoo/Documents/code/rails/rust-imdb-example` for a comprehensive example project.

## Requirements

- Rust 1.70 or later

## Running the Examples

```bash
cargo run --example basic
cd /Users/rajdeenoo/Documents/code/rails/rust-imdb-example
cargo run
```

## Running the Tests

```bash
cargo test
```

## Development

### Project Structure

```
rust-imdb-api/
├── src/
│   ├── client.rs      # Main API client
│   ├── constants.rs   # API constants
│   ├── error.rs       # Error types
│   ├── signer.rs      # AWS3-style authentication
│   └── lib.rs         # Public API
├── examples/
│   └── basic.rs       # Basic usage example
└── Cargo.toml
```

### Dependencies Used

- `reqwest` - HTTP client
- `serde` / `serde_json` - JSON serialization
- `hmac` / `sha2` - Cryptographic signing
- `base64` - Base64 encoding
- `regex` - IMDB ID validation
- `chrono` - Date/time handling
- `thiserror` - Error handling
- `url` - URL parsing
- `urlencoding` - URL encoding

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Bug reports and pull requests are welcome on GitHub.

## Acknowledgments

- Inspired by [imdb-apios](https://github.com/raj/imdb-apios) (Ruby)
