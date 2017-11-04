use bytes::{BufMut, BytesMut};
use tokio_io::codec::{Decoder};
use std::io::{Error, ErrorKind, Result};
use kraken_smtp::{ ClientCommand, EhloDomain, ServerReply, ServerReplyCode };
use super::{SmtpServerCodec, SmtpClientCodec };

impl Decoder for SmtpServerCodec {

    type Item = ClientCommand;

    type Error = Error;

    /// Decode a raw message sent by an SMTP client
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<ClientCommand>> {
        {
            let pr = buf.iter().position(|&b| b == b'\r');
            let pn = buf.iter().position(|&b| b == b'\r');

            let start = pr.or(pn);

            if let Some(i) = start {
                let line = buf.split_to(i);

            }
        }

        // This very naive implementation just looks for an end line character
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            buf.split_to(1);

            match ::std::str::from_utf8(&line) {
                Ok("quit") => Ok(Some(ClientCommand::Quit)),
                Ok(line) if line.len() == 3 => Ok(Some(ClientCommand::Reset)),
                Ok(line) => Ok(Some(ClientCommand::UnknownCommand(String::from(line)))),
                // As SMTP commands usually shouldn't contain invalid UTF-8 sequences but meh.
                Err(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid UTF-8 sequence"))
            }
        } else {
            Ok(None)
        }
    }

}

impl Decoder for SmtpClientCodec {

    type Item = ServerReply;

    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<ServerReply>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            Ok(None)
        } else {
            Ok(None)
        }
        //Ok(Some(ServerReply { reply_code: ServerReplyCode::GenericOk, reply_lines: vec![] }))
    }

}
