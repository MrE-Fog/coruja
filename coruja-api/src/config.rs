mod error;
mod spec;

use anyhow::Result;

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
}

#[derive(Debug)]
pub struct ServerConfig {
    /// server's binding address (may be a list of addresses separated by commas)
    address: String,
}

impl Config {
    pub fn from_env(prefix: &str) -> Result<Config> {
        let mut missing_required_variables: Vec<String> = Vec::new();

        // TODO refactor this code to capture the errors from getting env values from spec
        // then add them all to a list to present to the user.
        let config = Config {
            server: ServerConfig {
                address: spec::env_value_from_spec(spec::Spec::<String> {
                    key: &format!("{}{}", prefix, "SERVER_ADDRESS"),
                    rule: spec::Rule::Optional {
                        default: String::from("localhost:8080"),
                    },
                })?,
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
}