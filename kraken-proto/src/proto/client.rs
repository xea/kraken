use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::{ClientProto};
use kraken_smtp::{ClientCommand, ServerReply};
use std::io;

use super::SmtpClientProto;
use super::super::codec::SmtpServerCodec;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for SmtpClientProto {

    type Request = ServerReply;

    type Response = ClientCommand;

    type Transport = Framed<T, SmtpServerCodec>;

    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(SmtpServerCodec);

        Ok(transport)
    }
}
