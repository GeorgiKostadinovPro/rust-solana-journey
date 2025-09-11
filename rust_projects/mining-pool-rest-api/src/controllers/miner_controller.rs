use {
    actix_web::HttpResponse,
    actix_web::web::Json,

    crate::models::miner::*,
    crate::util::*
};

// REST API ENDPOINTS 

// GET - all active miners
#[get("/miners")]
pub async fn get_all() -> HttpResponse {
    let miners: Vec<Miner> = vec![];

    ResponseType::Ok(miners).get_response();
}

// GET - active miner by id
#[get("/miners/{id}")]
pub async fn get_by_id(id: String) -> HttpResponse {
    let miner: Option<Miner> = None;

    match miner {
        Some(payload) => ResponseType::Ok(payload).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Miner not found!".to_string())
        ).get_response()
    }
}

// POST - create a miner for a wallet
// miner is owned by a wallet, so the miner has to be created in specific wallet
#[post("/wallet/{id}/miners/create")]
pub async fn create(miner_request: Json<CreateMiner>) -> HttpResponse {
    let miner: Vec<Miner> = vec![];
    ResponseType::Ok(miner).get_response()
}
