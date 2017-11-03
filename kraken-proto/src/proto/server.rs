use futures::sink::Sink;
use futures::{Future, Stream};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::{ServerProto};
use kraken_smtp::{ClientCommand, ServerReply, ServerReplyCode};
use std::io;

use super::SmtpServerProto;
use super::super::codec::SmtpServerCodec;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for SmtpServerProto {

    type Request = ClientCommand;

    type Response = ServerReply;

    type Transport = Framed<T, SmtpServerCodec>;

    type BindTransport = Box<Future<Item=Self::Transport, Error=io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(SmtpServerCodec);

        let greeting = transport.send(ServerReply {
            reply_code: ServerReplyCode::GenericOk,
            reply_lines: vec![ String::from("Kraken SMTP"), ]
        });

        let handshake = greeting
            .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
            .and_then(|(line, transport)|{
                println!("{:?}", line);
                Ok(transport)
            });

        Box::new(handshake)
    }
}
