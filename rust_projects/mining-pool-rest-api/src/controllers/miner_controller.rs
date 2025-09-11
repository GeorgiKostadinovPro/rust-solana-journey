use {
    actix_web::HttpResponse,
    actix_web::web::Json,

    crate::models::*,
    crate::util::*
};

// REST API ENDPOINTS 

// GET - all active miners
#[get("/miners")]
pub async fn get_miners() -> HttpResponse {
    let miners: Vec<Miner> = vex![];
}
