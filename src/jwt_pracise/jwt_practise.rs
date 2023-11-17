use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

async fn generate_token() -> impl Responder {
    let my_claims = Claims {
        sub: "1234567890".to_owned(),
        company: "Acme".to_owned(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 3600, // 1 hour
    };
    let jwt_secret = b"secret";
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_secret)).unwrap();
    HttpResponse::Ok().body(format!("Token: {}", token))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(generate_token)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
