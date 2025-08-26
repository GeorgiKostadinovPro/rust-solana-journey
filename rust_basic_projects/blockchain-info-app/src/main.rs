// import dependencies
use tokio;

// use all macros from serde for serialization and deserialization
#[macro_use]
extern crate serde;

// register modules in the crate
mod node_api;
mod node_status;
mod node_address;
mod node_transaction;

// import modules 
// get from current crate
use crate::node_status::NodeStatus;
use crate::node_address::NodeAddress;
use crate::node_transaction::NodeTransaction;

// constants
const ACCOUNT_NOT_FOUND: &str = "ACCOUNT NOT FOUND";

async fn node_info_app(account: &str) {
    let node_status: NodeStatus = node_api::get_node_status().await;
    print!("\n\nQuerying: {} from chain: {}\n\n", &node_status.blockbook.coin, &node_status.backend.chain);

    let node_address: NodeAddress = node_api::get_node_address(&account).await;
    print!("\n\nAnalyzing tx for Bitcoin address {}\n\n", &node_address.address);
}

#[tokio::main]
async fn main() {
    let account = dotenv::var("ACCOUNT").expect(ACCOUNT_NOT_FOUND);
    node_info_app(&account).await;
}
