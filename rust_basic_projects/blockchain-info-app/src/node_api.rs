// import dependencies
use dotenv;
use reqwest;
use serde_json::Result;

// import modules
// get from current crate (added in main.rs)
use crate::node_status::NodeStatus;
use crate::node_address::NodeAddress;
use crate::node_transaction::NodeTransaction;

// Constants
const HOST_BASE: &str = "https://btcbook.nownodes.io/api/";

// Errors
const API_KEY_NOT_FOUND: &str = "API_KEY NOT FOUND";
const RESPONSE_NOT_SUCCESSFUL: &str = "RESPONSE NOT SUCCESSFUL";
const CONVERT_NOT_SUCCESSFUL: &str = "CONVERT NOT SUCCESSFUL";
const SERIALIZATION_NOT_SUCCESSFUL: &str = "SERIALIZATION NOT SUCCESSFUL";

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

/// @title get_node_status
/// @author GeorgiKostadinovPro
/// @notice get request fn for node status
/// @dev custom async get request fn for node status
pub async fn get_node_status() -> NodeStatus {
    let res = request(HOST_BASE).await;
    serde_json::from_str(&res).expect(SERIALIZATION_NOT_SUCCESSFUL)
}

/// @title get_node_address
/// @author GeorgiKostadinovPro
/// @notice get request fn for node address
/// @dev custom async get request fn for node address
pub async fn get_node_address(address: &str) -> NodeAddress {
    let req = [HOST_BASE, "v2/address/", &address].join("");
    let res = request(req).await;
    serde_json::from_str(&res).expect(SERIALIZATION_NOT_SUCCESSFUL)
}
