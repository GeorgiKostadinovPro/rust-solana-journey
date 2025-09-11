use {
    serde::{Serialize, Deserialize}
};

// JSON Payload (REST)
#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    // MH/s
    pub total_hash_rate: i32,
    pub total_mined_shares: i32,
    pub total_online_workers: i32,
    // wallet has many miners
    pub online_workers: Vec<Miner>
}

// POST Request Body - Create Wallet
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWallet {
    pub club_name: String
}

// Wallet DAO (DB Table Records)
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletDAO {
    pub address: String,
    pub club_name: String
}
