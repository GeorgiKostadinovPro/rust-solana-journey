use {
    serde::{Serialize, Deserialize}
};

// JSON Payload (REST)
#[derive(Debug, Serialize, Deserialize)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub nickname: String,
    pub club_name: String,
    // MH/s
    pub hash_rate: i32,
    pub mined_shares: i32
}

// POST Request Body - Create Miner
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMiner {
    pub nickname: String
}

// Miner DAO (DB Table Records)
#[derive(Debug, Serialize, Deserialize)]
pub struct MinerDAO {
    pub id: String,
    pub address: String,
    pub nickname: String,
    // MH/s
    pub hash_rate: i32,
    pub mined_shares: i32
}
