use actix_web::{
    App, 
    Error, 
    HttpRequest,
    HttpResponse, 
    HttpServer, 
    post, 
    Result,
    web};
use actix_cors::Cors;
use backend::models::Albums;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use jwt::{VerifyWithKey, SignWithKey};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde_derive::{Serialize, Deserialize};
use sha2::Sha256;
use std::env;
use std::collections::BTreeMap;
use sqlx::sqlite::SqlitePool;


#[derive(Deserialize)]
pub struct User {
    username: String 
}


#[derive(Serialize)]
pub struct SignupResponse {
    token: String 
}

#[post("/signup")]
async fn signup(form: web::Form<User>) -> Result<HttpResponse>{
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("username", form.username.clone());
    let token_str = claims.sign_with_key(&key).unwrap();
    let serialized = serde_json::to_string(&SignupResponse{token: token_str}).unwrap();
    Ok(HttpResponse::Ok().body(serialized))
}

#[derive(Debug, Deserialize)]
struct AlbumRequest  {
    id: i32 
}

#[post("/api/request")]
async fn index(
    pool: web::Data<SqlitePool>, 
    req: HttpRequest,
    album_req: web::Json<AlbumRequest>) -> Result<HttpResponse, Error> {

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let headers = req.headers();
    let token_str = headers.get("authorization").unwrap().to_str().unwrap();
    let _claims: BTreeMap<String, String> = match token_str.verify_with_key(&key) {
        Ok(claims) => claims, 
        Err(_) => return Ok(HttpResponse::Ok().body("Error with token")) 
    };

    let mut connection = pool.acquire().await.unwrap();
    let results = sqlx::query_as!(Albums, "SELECT * FROM albums WHERE album_id=?", album_req.id)
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
