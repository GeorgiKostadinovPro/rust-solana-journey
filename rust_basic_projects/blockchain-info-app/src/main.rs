// import dependencies
use tokio;

// use all macros from serde for serialization and deserialization
#[macro_use]
extern crate serde

// Register modules in the crate
mod node_api;
mod node_status;
mod node_address;
mod node_transaction;

// import modules 
// get from current crate
use crate::node_status::NodeStatus;
use crate::node_address::NodeAddress;
use crate::node_transaction::NodeTransaction;

async fn node_info_app(address: &str) {
    let node_status: NodeStatus = node_api::get_node_status().await;
    print!("\n\nQuerying: {} from chain: {}\n\n", &node_status.blockbook.coin, &node_status.backend.chain);

    let node_address: NodeAddress = node_api::get_node_address(&address).await;
    print!("\n\nAnalyzing tx for Bitcoin address {}\n\n", &node_address.address);
}

#[tokio:main]
async fn main() {
    node_info_app("").await;
}
