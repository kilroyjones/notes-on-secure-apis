use actix_web::{App, get, HttpServer, HttpResponse, Error, web};
use backend::models::Albums;
use dotenv::dotenv;
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

    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await

}