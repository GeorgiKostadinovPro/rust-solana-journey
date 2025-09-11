use {
    actix_web::HttpResponse,
    actix_web::web::Json,

    crate::models::wallet::*,
    crate::util::*
};

// REST API ENDPOINTS 

// GET - all active wallets
#[get("/wallets")]
pub async fn get_all() -> HttpResponse {
    let wallets: Vec<Wallet> = vec![];

    ResponseType::Ok(wallets).get_response();
}

// GET - active wallet by id
#[get("/wallets/{id}")]
pub async fn get_by_id(id: String) -> HttpResponse {
    let wallet: Option<Wallet> = None;

    match wallet {
        Some(payload) => ResponseType::Ok(payload).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Wallet not found!".to_string())
        ).get_response()
    }
}

// POST - create a miner for a wallet
// miner is owned by a wallet, so the miner has to be created in specific wallet
#[post("/wallets/create")]
pub async fn create(miner_request: Json<CreateWallet>) -> HttpResponse {
    let wallets: Vec<Wallet> = vec![];
    ResponseType::Ok(wallets).get_response()
}
