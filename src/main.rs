use actix_web::{web, App, HttpServer};
use crate::handlers::{hello, echo, manual_hello, post};
use rust_dock_lab::*;

mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = &mut establish_connection(); 

    HttpServer::new(|| {
        App::new()
            .service(hello::hello)
            .service(echo::echo)
            // .route("/post", web::post().to(post::create_post(connection, "asdf", "asdfasdf")))
            .route("/hey", web::get().to(manual_hello::manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
