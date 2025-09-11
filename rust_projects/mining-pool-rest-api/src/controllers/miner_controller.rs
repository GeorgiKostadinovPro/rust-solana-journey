use {
    actix_web::HttpResponse,
    actix_web::web::Json,

    crate::models::*,
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
#[get("/wallet/{id}/miners/create")]
pub async fn create_miner(miner_request: Json<CreateMiner>) -> HttpResponse {

}
