#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
    pub value: String,
    pub addresses: Vec<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: String,
    pub addresses: Vec<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeTx {
    pub txid: String,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>
}
