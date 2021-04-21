use std::pin::Pin;

use tokio::net::TcpStream;
// use tokio::io::{AsyncRead, AsyncWrite};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

use anyhow::{anyhow, Context, Result};

use openssl::nid::Nid;
use openssl::ssl::{SslConnector, SslConnectorBuilder, ConnectConfiguration, SslMethod, SslVerifyMode, Ssl};
use tokio_openssl::SslStream;
use openssl::stack::StackRef;
use openssl::x509::{X509Ref, X509};

pub async fn get_server_cert_chain(host: &str, port: &str, insecure: bool) -> Result<Vec<String>> {
    let ssl_connect_config: ConnectConfiguration = new_ssl_connect_config(insecure)?;

    let url = format!("{}:{}", host, port);
    let tcp_stream: TcpStream = TcpStream::connect(&url).await.context("io")?;

    let ssl: Ssl = ssl_connect_config.into_ssl(host)?;
    let mut ssl_stream = SslStream::new(ssl, tcp_stream)?;

    // https://github.com/cemoktra/ice-rs/blob/main/src/ssl.rs
    Pin::new(&mut ssl_stream)
        .connect()
        .await
        .map_err(|openssl_err| anyhow!("openssl: handshake: {}", openssl_err))?;

    let cert_stack: &StackRef<X509> = ssl_stream.ssl().peer_cert_chain().ok_or(anyhow!(
        "it was not possible to get certificate chain from server"
    ))?;

    let mut pem_list: Vec<String> = Vec::with_capacity(cert_stack.len());
    for cert in cert_stack.iter().flat_map(X509Ref::to_pem) {
        pem_list.push(String::from_utf8(cert)?);
    }
    Ok(pem_list)
}

/// Get a vec of certificates from a https server for a given url.
// TODO Make this function async!
// pub fn get_certs(url: &str, insecure: bool) -> Result<Vec<X509>> {
//     let tls_connect_config: ConnectConfiguration = new_ssl_connector(insecure)?;
//
//     let stream: TcpStream = TcpStream::connect(&url).context("io")?;
//
//     let stream: SslStream<TcpStream> = connector
//         .connect(&url, stream)
//         .map_err(|openssl_err| anyhow!("openssl: handshake: {}", openssl_err))?;
//
//     let cert_stack: &StackRef<X509> = stream.ssl().peer_cert_chain().ok_or(anyhow!(
//         "it was not possible to get certificate chain from server"
//     ))?;
//
//     let certs: Vec<X509> = cert_stack.iter().map(X509Ref::to_owned).collect();
//
//     Ok(certs)
// }

/// Creates a new SSL connect configuration
fn new_ssl_connect_config(insecure: bool) -> Result<ConnectConfiguration> {
    let mut connector_builder: SslConnectorBuilder =
        SslConnector::builder(SslMethod::tls()).context("openssl")?;

    if insecure {
        connector_builder.set_verify(SslVerifyMode::NONE);
    } else {
        connector_builder
            .set_default_verify_paths()
            .map_err(|openssl_error_stack| {
                anyhow!(
                    "openssl: connector builder: {:?}",
                    openssl_error_stack.errors()
                )
            })?;
    }

    Ok(connector_builder.build().configure()?)
}

/// Returns the common name of the certificate
fn _cert_common_name(cert: &X509) -> Result<String> {
    for name_entry in cert.subject_name().entries() {
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
