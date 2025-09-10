// import dependencies
use dotenv;
use reqwest;

// errors
const API_KEY_NOT_FOUND: &str = "API_KEY NOT FOUND";
const RESPONSE_NOT_SUCCESSFUL: &str = "RESPONSE NOT SUCCESSFUL";
const CONVERT_NOT_SUCCESSFUL: &str = "CONVERT NOT SUCCESSFUL";

/// @title request
/// @author GeorgiKostadinovPro
/// @notice request sender
/// @dev custom async request sender using reqwest HTTP client
pub async fn request(url: &str) -> String {
    // create a client object (preconfigured with components)
    let client = reqwest::Client::new();

    // extract api key from .env
    let api_key = dotenv::var("API_KEY").expect(API_KEY_NOT_FOUND);

    // requests CREATE, READ, UPDATE, DELETE
    client
        .get(url)
        .header("api-key", api_key)
        .send()
        .await
        .expect(RESPONSE_NOT_SUCCESSFUL)
        .text()
        .await
        .expect(CONVERT_NOT_SUCCESSFUL)
}
