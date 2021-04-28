mod api;
mod app;
mod config;
mod logging;
mod utils;

use actix_web::{web, App, HttpServer};
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    app::init();

    let app_state = app::State::new()?;
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
