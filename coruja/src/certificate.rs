use std::pin::Pin;

use tokio::net::TcpStream;

use anyhow::{anyhow, Context, Result};

use openssl::nid::Nid;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode, Ssl};
use tokio_openssl::SslStream;
use openssl::stack::StackRef;
use openssl::x509::{X509Ref, X509, X509NameRef};

pub async fn get_server_cert_chain(connector: &SslConnector, host: &str, port: &str, insecure: bool) -> Result<Vec<X509>> {
    let url = format!("{}:{}", host, port);
    let tcp_stream: TcpStream = TcpStream::connect(&url)
        .await
        .context("io")?;

    let mut ssl_connect_config = connector.configure()?
        .use_server_name_indication(true)
        .verify_hostname(true);

    if insecure {
        ssl_connect_config.set_verify(SslVerifyMode::NONE);
    }

    let ssl: Ssl = ssl_connect_config.into_ssl(host)?;
    let mut ssl_stream = SslStream::new(ssl, tcp_stream)?;

    // https://github.com/cemoktra/ice-rs/blob/main/src/ssl.rs
    Pin::new(&mut ssl_stream)
        .connect()
        .await
        .with_context(|| format!("openssl: async connect to {}:{}", host, port))?;

    let cert_stack: &StackRef<X509> = ssl_stream.ssl().peer_cert_chain().ok_or(anyhow!(
        "it was not possible to get certificate chain from server"
    ))?;

    let certs: Vec<X509> = cert_stack.iter()
        .map(X509Ref::to_owned)
        .collect();

    Ok(certs)
}

pub async fn get_server_cert_chain_as_string(connector: &SslConnector, host: &str, port: &str, insecure: bool) -> Result<Vec<String>> {

    let mut ssl_connect_config = connector.configure()?
        .use_server_name_indication(true)
        .verify_hostname(true);
    if insecure {
        ssl_connect_config.set_verify(SslVerifyMode::NONE);
    }

    let ssl: Ssl = ssl_connect_config.into_ssl(host)?;

    let url = format!("{}:{}", host, port);
    let tcp_stream: TcpStream = TcpStream::connect(&url)
        .await
        .context("io")?;

    let mut ssl_stream = SslStream::new(ssl, tcp_stream)?;

    // https://github.com/cemoktra/ice-rs/blob/main/src/ssl.rs
    Pin::new(&mut ssl_stream)
        .connect()
        .await
        .with_context(|| format!("openssl: async connect to {}:{}", host, port))?;

    let cert_stack: &StackRef<X509> = ssl_stream.ssl().peer_cert_chain().ok_or(anyhow!(
        "it was not possible to get certificate chain from server"
    ))?;

    let pem_list: Vec<String> = cert_stack_to_pem(cert_stack)?;
    Ok(pem_list)
}

// // TODO move this function out of the certificate module. Probably to the lib root
// pub fn new_ssl_client_context() -> Result<SslContext> {
//     let mut ssl_context_builder = SslContextBuilder::new(SslMethod::tls_client())
//         .context("ssl context")?;
//
//     ssl_context_builder.set_default_verify_paths()?;
//     ssl_context_builder.set_verify_depth(10);
//
//     Ok(ssl_context_builder.build())
// }

/// Creates a new SSL client connector (which can be configured before connecting to a server)
pub fn new_ssl_client_connector() -> Result<SslConnector> {
    let mut connector_builder = SslConnector::builder(SslMethod::tls())
        .context("openssl")?;

    connector_builder.set_default_verify_paths()?;
    connector_builder.set_verify_depth(10);

    Ok(connector_builder.build())
}

/// Returns the common name of the certificate
pub fn subject_common_name(cert: &X509) -> Result<String> {
    get_common_name(cert.subject_name())
}

/// Returns the common name of the certificate
pub fn issuer_common_name(cert: &X509) -> Result<String> {
    get_common_name(cert.issuer_name())
}

fn get_common_name(name: &X509NameRef) -> Result<String> {
    for name_entry in name.entries() {
        let asn1_object = name_entry.object();
        if asn1_object.nid() == Nid::COMMONNAME {
            return name_entry
                .data()
                .as_utf8()
                .map(|openssl_str| openssl_str.to_string())
                .map_err(|openssl_error_stack| {
                    anyhow!("openssl: utf-8 parsing: {:?}", openssl_error_stack.errors())
                });
        }
    }

    Err(anyhow!("common name not found"))
}

pub fn cert_stack_to_vec(cert_stack: &StackRef<X509>) -> Vec<X509> {
    cert_stack.iter()
        .map(X509Ref::to_owned)
        .collect()
}

pub fn cert_stack_to_pem(cert_stack: &StackRef<X509>) -> Result<Vec<String>> {
    let mut pem_list: Vec<String> = Vec::with_capacity(cert_stack.len());

    for cert in cert_stack.iter().flat_map(X509Ref::to_pem) {
        pem_list.push(String::from_utf8(cert)?);
    }

    Ok(pem_list)
}