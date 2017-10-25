extern crate futures;
extern crate kraken_proto;
extern crate native_tls;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_tls;

use auth::AuthService;
use config::ServerConfig;
use smtp::SmtpService;

mod auth;
mod config;
mod smtp;

fn main() {
    let config = ServerConfig::default();

    // The long-term idea is to split functionalities into separate processes each of which will
    // be responsible for a single main task (ie. networking, authenticating, queuing, etc).

    // Although this will not be implemented in the very beginning, separation of concerns will,
    // helping later restructuring into a multi-process architecture.


    let smtp_service = SmtpService::from(&config);
    let auth_service = AuthService::from(&config);


}
