const DEFAULT_CERTIFICATE_PATH: &'static str = "./server.key";
const DEFAULT_PRIVATE_KEY_PATH: &'static str = "./server.cert";

/// `ServerConfig` summarises the configuration options of a Kraken Server instance.
#[derive(Clone)]
pub struct ServerConfig {
    pub smtp_port: u16,
    pub submission_port: u16,
    pub tls_config: Option<TLSConfig>
}

impl Default for ServerConfig {

    fn default() -> Self {
        Self {
            // Port 25 is the standard port for sending messages from one MTA to another
            smtp_port: 25,
            // Port 587 is the standard port for submitting messages from a user client to an MTA
            submission_port: 587,
            // By default, we want to enable TLS (eg. STARTTLS) so we start with reasonable defaults
            tls_config: Some(TLSConfig::default())

        }
    }
}

/// `TLSConfig` contains all the configuration required for setting up a service that listens on a
/// TLS-encrypted port.
#[derive(Clone)]
pub struct TLSConfig {

    /// Path pointing to the file containing the certificate
    pub certificate_path: String,
    /// Path to the private key
    pub private_key_path: String
}

impl Default for TLSConfig {

    fn default() -> Self {
        TLSConfig {
            certificate_path: String::from(DEFAULT_CERTIFICATE_PATH),
            private_key_path: String::from(DEFAULT_PRIVATE_KEY_PATH)
        }
    }
}
