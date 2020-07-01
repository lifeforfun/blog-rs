use crate::ctl::response::ApiResponse;
use crate::libs::uuid::get_request_id;
use actix_web::{get, HttpRequest, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    uname: String,
    stat: bool,
}

#[get("/")]
pub(crate) async fn index(req: HttpRequest) -> impl Responder {
    let mut r = ApiResponse::new();
    r.set_data(HelloResponse {
        uname: String::from("🌜"),
        stat: true,
    })
    .set_error(10010, "lfsdlfsdl")
    .set_request_id(get_request_id(req))
    ;

    r.json()
}
