mod api;
mod app;
mod config;
mod logging;
mod utils;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use log::debug;

use config::Config;

#[actix_web::main]
async fn main() -> Result<()> {
    init();

    let app_state: app::State = {
        debug!("loading configuration from environment variables...");
        let config = Config::from_env("CORUJA_")?;

        app::State { config }
    };
    let addresses: Vec<String> = app_state
        .config
        .server()
        .address()
        .trim()
        .split(",")
        .map(String::from)
        .collect();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api").service(api::certificates::get_certificates))
    });

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
