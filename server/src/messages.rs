use serde_derive::Serialize;
use actix_web::web;

#[derive(Serialize)]
pub struct JsonResponse {
    res: String,
    data: String,
}

impl JsonResponse {
    pub fn new(res: &str, data: String) -> web::Json<JsonResponse> {
        web::Json(JsonResponse {
            res: res.into(),
            data: data 
        })
    }
}