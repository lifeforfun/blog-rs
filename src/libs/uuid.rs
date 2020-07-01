extern crate nanoid;
use actix_web::web::Query;
use actix_web::HttpRequest;
use nanoid::nanoid;
use serde::Deserialize;

pub fn get_request_id(req: HttpRequest) -> String {
    #[derive(Deserialize)]
    struct RequestId {
        request_id: String,
    }
    if let Ok(req_id) = Query::<RequestId>::from_query(req.query_string()) {
        req_id.request_id.clone()
    } else {
        (nanoid!(16, &[
            '1','2','3','4','5','6','7','8','9','0',
            'a','b','c','d','e','f','g','h','i','j',
            'k','l','m','n','o','p','q','r','s','t',
            'u','v','w','x','y','z','_','A','B','C',
            'D','E','F','G','H','I','J','K','L','M',
            'N','O','P','Q','R','S','T','U','V','W',
            'X','Y','Z',
            ]) as String).clone()
    }
}
