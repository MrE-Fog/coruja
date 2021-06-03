use crate::utils;
use actix_web::{get, http::header::Accept, web, HttpResponse};
use coruja;
use serde::Deserialize;
use openssl::x509::X509;

#[derive(Deserialize)]
pub struct SiteQueryParams {
    host: String,
    port: Option<String>,
    insecure: Option<bool>,
}

mod payload {
    use serde::Serialize;
    use openssl::x509::X509;
    use openssl::asn1::Asn1Time;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Certificate {
        subject: Subject,
        issuer: Issuer,
        not_before: String,
        not_after: String,
        expires_in_days: i32,
        serial_number: String,
        pem: String,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subject {
        common_name: String,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Issuer {
        common_name: String,
    }

    impl Certificate {
        pub fn try_from_x509(crt: &X509) -> anyhow::Result<Self> {
            let subject = Subject {
                common_name: coruja::certificate::subject_common_name(crt)?,
            };
            let issuer = Issuer {
                common_name: coruja::certificate::issuer_common_name(crt)?,
            };
            let not_before: String = crt.not_before().to_string();
            let not_after: String = crt.not_after().to_string();
            let now: Asn1Time = Asn1Time::days_from_now(0)?;
            let expires_in_days: i32 = now.diff(crt.not_after())?.days;
            let serial_number: String = crt.serial_number().to_bn()?.to_string();
            let pem: String = String::from_utf8(crt.to_pem()?)?;

            Ok(Certificate {
                subject,
                issuer,
                not_before,
                not_after,
                expires_in_days,
                serial_number,
                pem,
            })
        }
    }
}

#[get("/certificates")]
pub async fn get_certificates(
    app_state: web::Data<crate::app::State>,
    accept: web::Header<Accept>,
    query: web::Query<SiteQueryParams>,
) -> Result<HttpResponse, actix_web::Error> {

    let host = &query.host;
    let port = query.port.clone().unwrap_or_else(|| String::from("443"));
    let insecure = query.insecure.unwrap_or(false);

    let accept_has_application_json: bool =
        utils::accepts_mime_type(&accept, &mime::APPLICATION_JSON);

    if accept_has_application_json {
        let cert_chain_result: anyhow::Result<Vec<X509>> =
            coruja::certificate::get_server_cert_chain(&app_state.ssl_client_connector, host, &port, insecure)
                .await;

        // https://users.rust-lang.org/t/using-actix-and-anyhow-together/40774
        let certs: Vec<X509> = match cert_chain_result {
            Ok(certs) => certs,
            Err(err) => {
                // TODO diferenciar entre erros inesperados (5XX) de erros de negócio (4XX)
                return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)));
            }
        };

        let mut body: Vec<payload::Certificate> = Vec::with_capacity(certs.len());
        for crt in certs {
            // https://users.rust-lang.org/t/using-actix-and-anyhow-together/40774
            let p = match payload::Certificate::try_from_x509(&crt) {
                Ok(x) => x,
                Err(err) => {
                    return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)));
                }
            };
            body.push(p);
        }

        Ok(HttpResponse::Ok().json(body))
    } else {
        let cert_chain_result: anyhow::Result<Vec<String>> =
            coruja::certificate::get_server_cert_chain_as_string(&app_state.ssl_client_connector, host, &port, insecure)
                .await;

        let certs: Vec<String> = match cert_chain_result {
            Ok(certs) => certs,
            Err(err) => {
                // TODO diferenciar entre erros inesperados (5XX) de erros de negócio (4XX)
                return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)));
            }
        };
        Ok(HttpResponse::Ok().body(certs.join("")))
    }
}
