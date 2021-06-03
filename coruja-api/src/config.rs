mod error;
mod spec;

use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct Config {
    server: ServerConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// server's binding address (may be a list of addresses separated by commas)
    address: String,

    /// sets how many workers the server should spawn
    workers: usize,
}

impl Config {
    pub fn from_env(prefix: &str) -> Result<Config> {
        let mut bad_variables: Vec<String> = Vec::new();

        let server_address_spec = spec::Spec::<String> {
            key: &format!("{}{}", prefix, "SERVER_ADDRESS").to_owned(),
            rule: spec::Rule::Optional {
                default: String::from("localhost:8080"),
            },
        };
        let server_address = match spec::env_value_from_spec(server_address_spec.clone()) {
            Ok(v) => Some(v),
            Err(err) => {
                eprintln!("{:?}", err);
                bad_variables.push(server_address_spec.key.to_string());
                None
            }
        };

        let server_workers_spec = spec::Spec::<usize> {
            key: &format!("{}{}", prefix, "SERVER_WORKERS"),
            rule: spec::Rule::Optional {
                default: 4usize,
            }
        };
        let server_workers = match spec::env_value_from_spec(server_workers_spec.clone()) {
            Ok(v) => Some(v),
            Err(err) => {
                eprintln!("{:?}", err);
                bad_variables.push(server_workers_spec.key.to_string());
                None
            }
        };

        if !bad_variables.is_empty() {
            return Err(anyhow!(error::Error::MissingRequiredVaribles {
                missing_variables: bad_variables,
            }));
        }

        // then add them all to a list to present to the user.
        // OBS.: all this unwraps are safe because any none value will trigger the above
        // "early" return
        let config = Config {
            server: ServerConfig {
                address: server_address.unwrap(),
                workers: server_workers.unwrap(),
            },
        };

        Ok(config)
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
}

impl ServerConfig {
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn workers(&self) -> usize {
        self.workers
    }
}
