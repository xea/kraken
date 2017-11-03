use bytes::{BufMut, BytesMut, BigEndian};
use kraken_smtp::{ServerReply, ClientCommand};
use tokio_io::codec::Encoder;
use std::io::{Error, Result};

use super::{SmtpServerCodec, SmtpClientCodec };

impl Encoder for SmtpServerCodec {
    type Item = ServerReply;
    type Error = Error;

    fn encode(&mut self, server_reply: ServerReply, buf: &mut BytesMut) -> Result<()> {
        let line_count = server_reply.reply_lines.len();

        if line_count > 0 {
            // For multi-line responses all lines except the last one need to be separated from the
            // reply code with a dash '-' character.
            for line in server_reply.reply_lines[0..((line_count - 1).max(0))].iter() {
                let current_line = format!("{}-{}\r\n", server_reply.reply_code, line);

                buf.extend(current_line.as_bytes());
            }

            // The last line has to be separated with a single space
            if line_count > 0 {
                let last_line = format!("{} {}\r\n", server_reply.reply_code, server_reply.reply_lines[line_count - 1]);

                buf.extend(last_line.as_bytes());
            }
        } else {
            buf.extend(format!("{}\r\n", server_reply.reply_code).as_bytes());
        }

        Ok(())
    }
}

impl Encoder for SmtpClientCodec {
    type Item = ClientCommand;
    type Error = Error;

    fn encode(&mut self, client_command: ClientCommand, buf: &mut BytesMut) -> Result<()> {
        Ok(())
    }
}
