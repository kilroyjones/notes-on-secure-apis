use actix_web::{App, get, HttpServer, HttpResponse, Error, web};
use actix_cors::Cors;
use backend::models::Albums;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;
use sqlx::sqlite::SqlitePool;


#[get("/albums/{id}")]
async fn index(
    path: web::Path<u32>,
    pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {

    let id = path.into_inner();
    let mut connection = pool.acquire().await.unwrap();
    let results = sqlx::query_as!(Albums, "SELECT * FROM albums WHERE album_id=?", id)
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

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("localhost-key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("localhost.pem").unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(index)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind_openssl("127.0.0.1:3000", builder)?
    .run()
    .await

}
