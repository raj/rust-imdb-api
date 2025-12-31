use imdb_api::ImdbClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ImdbClient::new();

    println!("=== IMDB API Example ===\n");

    // Example 1: Validate IMDB ID
    println!("1. Validating IMDB IDs:");
    println!("   tt0111161 (The Shawshank Redemption): {}", imdb_api::validate_imdb_id("tt0111161"));
    println!("   nm0000151 (Christian Bale): {}", imdb_api::validate_imdb_id("nm0000151"));
    println!("   invalid: {}", imdb_api::validate_imdb_id("invalid"));
    println!();

    // Example 2: Check if title exists
    println!("2. Checking if title exists:");
    let exists = client.title_exists("tt0111161").await?;
    println!("   tt0111161 exists: {}", exists);
    println!();

    // Example 3: Get title information
    println!("3. Getting title information for tt0111161 (The Shawshank Redemption):");
    match client.get_title("tt0111161").await {
        Ok(title) => {
            println!("   Title: {:?}", serde_json::to_string_pretty(&title).unwrap());
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 4: Search for a title
    println!("4. Searching for 'The Dark Knight':");
    match client.search("The Dark Knight").await {
        Ok(results) => {
            println!("   Results: {:?}", serde_json::to_string_pretty(&results).unwrap());
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 5: Search for a person
    println!("5. Searching for 'Christian Bale':");
    match client.search("Christian Bale").await {
        Ok(results) => {
            if let Some(d) = results.get("d") {
                if let Some(arr) = d.as_array() {
                    println!("   Found {} results", arr.len());
                    for (i, item) in arr.iter().take(5).enumerate() {
                        if let Some(name) = item.get("l").and_then(|l| l.as_str()) {
                            println!("   {}: {}", i + 1, name);
                        }
                    }
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 6: Get popular titles
    println!("6. Getting popular titles:");
    match client.get_popular_titles().await {
        Ok(titles) => {
            println!("   Popular titles: {:?}", serde_json::to_string_pretty(&titles).unwrap());
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 7: Get popular TV shows
    println!("7. Getting popular TV shows:");
    match client.get_popular_shows().await {
        Ok(shows) => {
            println!("   Popular shows: {:?}", serde_json::to_string_pretty(&shows).unwrap());
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    Ok(())
}
