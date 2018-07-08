extern crate kraken;

use kraken::config::ServerConfig;
use kraken::server::ThreadedServer;

pub fn main() {
    let config = ServerConfig::new();

    println!("Welcome to {}", config.smtpBanner);

    let server = ThreadedServer::new();
}
