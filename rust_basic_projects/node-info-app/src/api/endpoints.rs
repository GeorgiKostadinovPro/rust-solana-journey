// import dependencies
use serde_json::Result;

// import modules
// get from current crate (added in main.rs)
use crate::models::{node_status::NodeStatus, node_address::NodeAddress, node_tx::NodeTx};

use super::client::request;

// constants
const HOST_BASE: &str = "https://btcbook.nownodes.io/api/";

// errors
const SERIALIZATION_NOT_SUCCESSFUL: &str = "SERIALIZATION NOT SUCCESSFUL";

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
    let url = [HOST_BASE, "v2/address/", &address].join("");
    let res = request(&url).await;
    serde_json::from_str(&res).expect(SERIALIZATION_NOT_SUCCESSFUL)
}

/// @title get_node_tx
/// @author GeorgiKostadinovPro
/// @notice get request fn for node tx
/// @dev custom async get request fn for node tx
pub async fn get_node_tx(tx_id: &str) -> NodeTx {
    let url = [HOST_BASE, "v2/tx/", &tx_id].join("");
    let res = request(&url).await;
    serde_json::from_str(&res).expect(SERIALIZATION_NOT_SUCCESSFUL)
}
