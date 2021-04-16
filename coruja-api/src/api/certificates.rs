use actix_web::{web, Responder, get};
use serde::Deserialize;

use coruja;

#[derive(Deserialize)]
struct SiteQueryParams {
    host: String,
    port: String,
}

#[get("/certificates")]
pub async fn get_certificates(query: web::Query<SiteQueryParams>) -> impl Responder {
    // TODO descobrir como mapear erro do anyhow em uma resposta do actix
    // coruja::certificate::get_cert_chain_rustls(&query.host, &query.port).unwrap();
    let certs: Vec<String> = coruja::certificate::get_cert_chain_openssl(&query.host, &query.port, true)
        .unwrap();
    certs.join("")
}
