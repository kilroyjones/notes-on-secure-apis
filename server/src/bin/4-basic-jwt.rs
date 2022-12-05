use actix_web::{
    App, 
    dev, 
    Error, 
    get, 
    FromRequest, 
    HttpRequest,
    HttpServer, 
    HttpResponse, 
    post, 
    put,
    Result,
    web};
use actix_cors::Cors;
use backend::models::Albums;
use dotenv::dotenv;
use futures_util::future::{ok, err, Ready};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde_derive::Deserialize;
use std::env;
use sqlx::sqlite::SqlitePool;


#[derive(Deserialize)]
pub struct User {
    id: i64
}

#[post("/signup")]
async fn signup(form: web::Form<User>) -> Result<HttpResponse>{
    println!("{:?}", form.id);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
       .body(format!("Worked")))
}


#[derive(Debug, Deserialize)]
struct Thing {
    name: String
}

impl FromRequest for Thing {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        println!("{:?}", req);
        ok(Thing { name: "thingy".into() })
    }
}

#[get("/albums/{id}")]
async fn index(
    path: web::Path<u32>,
    pool: web::Data<SqlitePool>, 
    thing: Option<Thing>) -> Result<HttpResponse, Error> {

    match thing {
        Some(_) => println!("ASDFsa"),
        None => println!("sdfadsf")
    };

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
            .service(signup)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind_openssl("localhost:3000", builder)?
    .run()
    .await

}
