mod api;
mod config;
mod logging;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use log::debug;

#[actix_web::main]
async fn main() -> Result<()> {
    init();

    debug!("loading configuration from environment variables...");
    let cfg = config::Config::from_env("CORUJA_")?;

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api").service(api::certificates::get_certificates))
    });

    let addresses: Vec<&str> = cfg.server().address().trim().split(",").collect();
    for address in addresses {
        server = server.bind(address)?;
    }

    server.run().await?;

    Ok(())
}

fn init() {
    logging::init();
    coruja::init();
}
