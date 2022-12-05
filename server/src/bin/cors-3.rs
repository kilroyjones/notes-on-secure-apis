use actix_web::{App, post, HttpServer, HttpResponse, Error, web};
use actix_cors::Cors;
use backend::models::Albums;
use dotenv::dotenv;
use std::env;
use sqlx::sqlite::SqlitePool;


use serde_derive::Deserialize;
#[derive(Deserialize, Debug)]
pub struct AlbumRequest {
    id: i32,
}

#[post("/index")]
async fn index(
    album_request: web::Data<AlbumRequest>,
    pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {

    let mut connection = pool.acquire().await.unwrap();
    let results = sqlx::query_as!(Albums, "SELECT * FROM albums WHERE album_id=?", album_request.id)
        .fetch_all(&mut connection)
        .await.unwrap();

    let serialized = serde_json::to_string(&results).unwrap();
    Ok(HttpResponse::Ok().body(serialized))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_path = env::var("DATABASE_URL").unwrap();
    let pool = SqlitePool::connect(&db_path).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(index)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await

}
