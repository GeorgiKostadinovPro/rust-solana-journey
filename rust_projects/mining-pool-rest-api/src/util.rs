use {
    actix_web::HttpResponse,
    serde::{Serialize, Deserialize}
};

pub enum ResponseType<T> {
    Ok(T),
    NotFound(T),
    Success(T)
}

// only for serializable types
impl<T: Serialize> ResponseType<T> {
    pub fn get_response(&self) -> HttpResponse {
        match self {
            ResponseType::Ok(payload) -> HttpResponse::Ok()
                .content_type("application/json")
                .json(payload),
            ResponseType::NotFound(payload) -> HttpResponse::BadRequest()
                .content_type("application/json")
                .json(payload),
            ResponseType::Success(payload) -> HttpResponse::Created()
                .content_type("application/json")
                .json(payload)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotFound {
    message: String
}

impl NotFound {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}