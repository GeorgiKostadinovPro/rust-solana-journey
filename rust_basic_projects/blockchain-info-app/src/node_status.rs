#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blockbook {
    pub coin: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "CamelCase")]
pub struct Backend {
    pub chain: String
}

pub struct NodeStatus {
    pub blockbook: Blockbook,
    pub backend: Backend
}
