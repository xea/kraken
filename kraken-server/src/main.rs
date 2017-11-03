extern crate futures;
extern crate kraken_proto;
extern crate kraken_smtp;
extern crate native_tls;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_tls;

use config::ServerConfig;
use smtp::SmtpServer;

mod auth;
mod config;
mod smtp;

fn main() {
    let _ = ServerConfig::default();

    // The long-term idea is to split functionalities into separate processes each of which will
    // be responsible for a single main task (ie. networking, authenticating, queuing, etc).
    // This will initially be implemented as threads within the same process

    //let smtp_thread = thread::spawn(move || {
        let mut smtp_server = SmtpServer;

        smtp_server.run();
    //});

    //let _ = smtp_thread.join();

    println!("Server exited");
}
