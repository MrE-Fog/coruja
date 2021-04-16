mod config;
mod logging;
mod api;

use log::{debug};
use anyhow::Result;
use actix_web::{get, web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> Result<()> {
    logging::init();

    debug!("loading configuration from environment variables...");
    let cfg = config::Config::from_env("CORUJA_")?;

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api")
                .service(api::certificates::get_certificates)
            )
    });

    let addresses: Vec<&str> = cfg.server().address()
        .trim()
        .split(",")
        .collect();
    for address in addresses {
        server = server.bind(address)?;
    }

    server
        .run()
        .await?;

    Ok(())
}

#[get("/hello/{name}")]
async fn index(path: web::Path<String>) -> impl Responder {
    format!("Hello {}!", path)
}
