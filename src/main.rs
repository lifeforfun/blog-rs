#[allow(unused)]

mod ctl;
mod libs;
mod pages;

use actix_web::{web, App, HttpServer};
use pages::*;
use actix_session::CookieSession;

fn main() {
    let _ = server();
}

#[actix_rt::main]
async fn server() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .wrap(
                CookieSession::signed(&[0;64])
                    .secure(false),
            )
            .service(
                web::scope("")
                    .service(home::index::index)
            )
            .service(
                web::scope("/admin")
                    .service(admin::index::index)
            )
    )
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
