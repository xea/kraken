/// By default, we want to listen on an unprivileged port so that the program doesn't require elevated privileges.
const DEFAULT_SMTP_LISTEN_PORT: u16 = 9025;
const DEFAULT_SMTP_BANNER: &'static str = "Kraken SMTP Server";

/// Run-time configuration for the server instance. 
pub struct ServerConfig {
    pub listenPort: u16,
    pub smtpBanner: String
}

impl ServerConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listenPort: DEFAULT_SMTP_LISTEN_PORT,
            smtpBanner: String::from(DEFAULT_SMTP_BANNER)
        }
    }
}