use crate::models::post::{NewPost, Post};
use crate::schema::posts;
use diesel::SelectableHelper;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use actix_web::{HttpResponse, Responder};

pub async fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> impl Responder {
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post");
    HttpResponse::Ok().body("asdfasdf")
}