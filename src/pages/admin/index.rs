use actix_web::{HttpRequest, Responder};
use actix_session::Session;
use crate::ctl::auth::UserAuth;
use crate::ctl::response::ApiResponse;
use crate::libs::uuid::get_request_id;

#[get("/admin")]
pub fn index(req: HttpRequest, sess: Session) -> impl Responder
{
    let _ = UserAuth::check_role(sess, "admin")?;

    let mut r = ApiResponse::new();

    r.set_request_id(get_request_id(req));

    r.json()
}