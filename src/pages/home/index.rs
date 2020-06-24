use actix_web::{get, Responder, HttpResponse};

#[get("/hello")]
pub(crate) async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}