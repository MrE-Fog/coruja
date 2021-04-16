pub mod certificate;

pub fn init() {
    openssl_probe::init_ssl_cert_env_vars();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
