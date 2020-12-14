use std::net::{IpAddr, TcpListener};
use std::sync::Arc;
use std::io::{Write, BufReader, Error};
use std::fs::File;
use rustls::{AllowAnyAnonymousOrAuthenticatedClient, RootCertStore, ServerSession, Stream};
use rustls::ServerConfig as TLSConfig;

pub enum TLSError {
    KeyFileError(Error),
    PrivateKeyError,
    CertFileError(Error),
    CertificateError,
    EmptyKeyStore,
    TLSImplError(rustls::TLSError)
}

fn tls_config(security_config: &SecurityConfig) -> Result<TLSConfig, TLSError> {
    let private_key = File::open(security_config.server_key_path)
        .map_err(TLSError::KeyFileError)
        .map(BufReader::new)
        .and_then(|mut reader| rustls::internal::pemfile::pkcs8_private_keys(&mut reader)
            .map_err(|_| TLSError::PrivateKeyError))
        .and_then(|mut keys| keys.pop()
            .ok_or(TLSError::EmptyKeyStore))?;

    let certificates = File::open(security_config.certificate_path)
        .map_err(TLSError::CertFileError)
        .map(BufReader::new)
        .and_then(|mut reader| rustls::internal::pemfile::certs(&mut reader)
            .map_err(|_| TLSError::CertificateError))?;

    let verifier = AllowAnyAnonymousOrAuthenticatedClient::new(RootCertStore::empty());
    let mut tls_config = TLSConfig::new(verifier);

    let _ = tls_config.set_single_cert(certificates, private_key)
        .map_err(|err| TLSError::TLSImplError(err))?;

    Ok(tls_config)
}

pub fn main() {
    let server_config = ServerConfig::default();
    let tls_config = tls_config(&server_config.security)
        .ok()
        .expect("Unable to load TLS settings");

    //let config_rc = Arc::new(tls_config);

    match TcpListener::bind("0.0.0.0:25") {
        Ok(server_socket) => main_loop(server_socket, tls_config),
        Err(err) => eprintln!("Could not bind to port 25: {}", err)
    };
}

fn main_loop(server_socket: TcpListener, tls_config: TLSConfig) {
    let config_rc = Arc::new(tls_config);

    loop {
        match server_socket.accept() {
            Ok(mut client) => {
                let mut server_session = ServerSession::new(&config_rc);
                let mut stream = Stream::new(&mut server_session, &mut client.0);

                let _ = stream.write("Hello".as_bytes())
                    .and_then(|_| stream.flush());
            },
            Err(err) => eprintln!("Could not accept client: {}", err)
        }
    }
}

#[derive(Default)]
struct ServerConfig<'a> {
    pub network: NetworkConfig,
    pub security: SecurityConfig<'a>
}

struct NetworkConfig {
    pub listen_addr: IpAddr,
    pub listen_port: u16
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addr: [ 0, 0, 0, 0 ].into(),
            listen_port: 25
        }
    }
}

struct SecurityConfig<'a> {
    server_key_path: &'a str,
    certificate_path: &'a str
}

impl<'a> Default for SecurityConfig<'a> {
    fn default() -> Self {
        Self {
            server_key_path: "kraken.key.pem",
            certificate_path: "kraken.cert.pem"
        }
    }
}