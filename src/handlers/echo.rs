use actix_web::{post, HttpResponse, Responder};

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}