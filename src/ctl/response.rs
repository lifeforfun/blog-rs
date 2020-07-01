use actix_web::{HttpResponse, Responder};
use serde::Serialize;
use actix_session::Session;
use failure::Error;

#[derive(Serialize)]
pub struct ApiResponse<'a, T>
where
    T: Serialize,
{
    errno: usize,
    errmsg: &'a str,
    request_id: Option<String>,
    data: Option<T>,
}

impl<'a, T> ApiResponse<'a, T>
where
    T: Serialize,
{
    pub fn new() -> ApiResponse<'a, T> {
        ApiResponse {
            errno: 0,
            errmsg: "success",
            request_id: None,
            data: None,
        }
    }

    pub fn check_role(&mut self, sess:Session, role: &str) -> Result<&mut Self, Error> {

    }

    pub fn set_error(&mut self, errno: usize, errmsg: &'a str) -> &mut Self {
        self.errno = errno;
        self.errmsg = errmsg;
        self
    }

    pub fn set_data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn set_request_id(&mut self, req_id: String) -> &mut Self {
        self.request_id = Some(req_id);
        self
    }

    pub fn get_request_id(&self) -> Option<String> {
        self.request_id.clone()
    }

    pub fn json(&self) -> impl Responder {
        HttpResponse::Ok().json(self)
    }
}
