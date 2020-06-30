use actix_web::{get, Responder};
use crate::ctl::response::json_response;
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse{
    uname: String,
    stat: bool,
}

#[get("/hello")]
pub(crate) async fn index() -> impl Responder {
    json_response(None::<HelloResponse>, None)
}