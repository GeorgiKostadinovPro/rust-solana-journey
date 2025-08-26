// import dependencies
use dotenv;
use reqwest;
use tokio;
use serde_json::Result;

// import modules // get from current crate (added in main.rs)
use crate::node_status::NodeStatus;
use crate::node_address::NodeAddress;
use crate::node_transaction::NodeTransaction;

const HOST_BASE: &str = "https://btcbook.nownodes.io/api/";
const API_KEY_NOT_FOUND: &str = "API_KEY NOT FOUND";
const RESPONSE_NOT_SUCCESSFUL: &str = "RESPONSE NOT SUCCESSFUL";
const CONVERT_NOT_SUCCESSFUL: &str = "CONVERT NOT SUCCESSFUL";

/// @title request
/// @author GeorgiKostadinovPro
/// @notice async request sender
/// @dev custom request sender using reqwest HTTP client
#[tokio:main]
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

pub async fn node_status_request() -> NodeStatus {

}
