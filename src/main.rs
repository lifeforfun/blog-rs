mod pages;

use actix_web::{HttpServer, App};
use pages::*;

fn main() {
    let _ = server();
}

#[actix_rt::main]
async fn server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home::index::index)
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}