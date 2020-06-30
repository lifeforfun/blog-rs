
use actix_web::{Responder, HttpResponse};
use serde::Serialize;

pub (crate) struct ApiError<'a> {
    errno: usize,
    errmsg: &'a str,
}

#[derive(Serialize)]
pub (crate) struct ApiResponse<'a, T>
where
    T: Serialize,
{
    errno: usize,
    errmsg: &'a str,
    data: Option<T>,
}

pub (crate) fn json_response<T>(data: Option<T>, error: Option<ApiError>) -> impl Responder
where
    T: Serialize,
{
    let r = match error {
        Some(e) => ApiResponse{
            errno: e.errno,
            errmsg: e.errmsg,
            data,
        },
        None => ApiResponse{
            errno: 0,
            errmsg: "success",
            data,
        },
    };

    HttpResponse::Ok()
        .json(r)
}