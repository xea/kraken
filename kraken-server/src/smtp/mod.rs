pub mod service;

use super::config::ServerConfig;
use kraken_proto::proto::SmtpServerProto;
use tokio_proto::TcpServer;

use self::service::SmtpService;

pub struct SmtpServer;

impl SmtpServer {
    pub fn run(&mut self) -> () {
        let address = "0.0.0.0:9925".parse().unwrap();

        let server_proto = SmtpServerProto;

        let server = TcpServer::new(server_proto, address);

        server.serve(move || {
            println!("Serving the dark lord");
            Ok(SmtpService)
        });
    }
}
