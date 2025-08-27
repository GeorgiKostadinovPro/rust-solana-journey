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
use {
    crate::node_status::NodeStatus,
    crate::node_address::NodeAddress,
    crate::node_transaction::NodeTransaction,
    dotenv,
    std::{io, thread, time}
};

// constants
const ACCOUNT_NOT_FOUND: &str = "ACCOUNT NOT FOUND";

async fn node_info_app(account: &str) {
    let node_status: NodeStatus = node_api::get_node_status().await;
    print!("\n\nQuerying: {} from chain: {}\n\n", &node_status.blockbook.coin, &node_status.backend.chain);

    let node_address: NodeAddress = node_api::get_node_address(&account).await;
    print!("\n\nAnalyzing tx for Bitcoin address {}\n\n", &node_address.address);

    let pause_time = time::Duration::from_millis(3000);
    thread::sleep(pause_time);

    println!("\nThe account has {} txs", &node_address.txids.len());

    println!("To query the txs please choose (y/n): \n");

    let mut cmd = String::new();

    io::stdin().read_line(&mut cmd);

    if cmd.trim().eq("n") {
        println!("\nYou exited the system\n");
        return;
    }

    println!("\nQuerying tx please wait...\n");

    thread::sleep(pause_time);

    // if vector is empty return None 
    // otherwise beautify the output
    println!("{:#?}", &node_address.txids);

    println!("\nGet info for specific tx id? Please type the id below: \n");

    io::stdin().read_line(&mut cmd);
}

#[tokio::main]
async fn main() {
    let account = dotenv::var("ACCOUNT").expect(ACCOUNT_NOT_FOUND);
    node_info_app(&account).await;
}
