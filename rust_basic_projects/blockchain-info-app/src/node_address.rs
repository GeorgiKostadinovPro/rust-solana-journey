#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    pub address: String,
    pub txids: Vec<String>
}
