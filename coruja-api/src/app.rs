use log::debug;
use anyhow::Result;
use openssl::ssl::{SslConnector};

use crate::logging;
use crate::config::Config;

#[derive(Clone)]
pub struct State {
    pub config: Config,
    // pub ssl_client_context: SslContext,
    pub ssl_client_connector: SslConnector,
}

impl State {
    pub fn new() -> Result<Self> {
        debug!("loading configuration from environment variables...");
        let config = Config::from_env("CORUJA_")?;

        // debug!("creating default ssl client context...");
        // let ssl_client_context = coruja::certificate::new_ssl_client_context()?;

        debug!("creating default ssl connector...");
        let ssl_client_connector = coruja::certificate::new_ssl_client_connector()?;

        Ok(State {
            config,
            // ssl_client_context,
            ssl_client_connector,
        })
    }
}

pub fn init() {
    logging::init();
    coruja::init();
}
