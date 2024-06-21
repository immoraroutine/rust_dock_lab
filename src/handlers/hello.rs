use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    println!("asdfasdfasdfasdf");
    println!("asdf");
    HttpResponse::Ok().body("Hello world!")
}