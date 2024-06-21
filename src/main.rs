use actix_web::{web, App, HttpServer};
use crate::handlers::{hello, echo, manual_hello};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello::hello)
            .service(echo::echo)
            .route("/hey", web::get().to(manual_hello::manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
