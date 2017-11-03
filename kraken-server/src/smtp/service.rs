use futures::future::{result, Future};
use kraken_smtp::{ClientCommand, ServerReply, ServerReplyCode};
use tokio_service::{Service};
use std::io;


pub struct SmtpService;

impl SmtpService {
}

impl Service for SmtpService {
    type Request = ClientCommand;
    type Response = ServerReply;

    type Error = io::Error;

    type Future = Box<Future<Item=ServerReply, Error=io::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        println!("SmtpService.call: {:?}", request);
        // TODO process request here
        let reply = ServerReply {
            reply_code: ServerReplyCode::GenericOk,
            reply_lines: vec![]
        };

        Box::new(result(Ok(reply)))
    }
}
