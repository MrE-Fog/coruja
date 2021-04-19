use crate::utils;
use actix_web::{get, http::header::Accept, web, HttpResponse};
use coruja;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SiteQueryParams {
    host: String,
    port: Option<String>,
    insecure: Option<bool>,
}

mod payload {
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Certificate {
        subject: String,
        issuer: String,
        not_after: std::time::Duration,
        not_before: std::time::Duration,
        expires_in: std::time::Duration,
        serial_number: String,
        pem: String,
    }
}

// impl Responder
#[get("/certificates")]
pub async fn get_certificates(
    accept: web::Header<Accept>,
    query: web::Query<SiteQueryParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let cert_chain_result = web::block(move || {
        // TODO descobrir como mapear erro do anyhow em uma resposta do actix
        let host = &query.host;
        let port = query.port.clone().unwrap_or_else(|| String::from("443"));
        let insecure = query.insecure.unwrap_or(false);
        coruja::certificate::get_server_cert_chain(host, &port, insecure)
    }).await?;

    let certs: Vec<String> = match cert_chain_result {
        Ok(certs) => certs,
        Err(err) => {
            // TODO diferenciar entre erros inesperados (500) de erros de negócio (4XX)
            return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)));
        }
    };

    // TODO retornar bad request caso Accept não seja text/plain ou application/json?
    let _accept_has_application_json: bool =
        utils::accepts_mime_type(&accept, &mime::APPLICATION_JSON);

    // if accept_has_application_json {
    //     HttpResponse::Ok().json(joined_certs)
    // } else {
    //     joined_certs
    // }
    Ok(HttpResponse::Ok().body(certs.join("")))
}
