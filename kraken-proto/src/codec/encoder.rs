use bytes::{BufMut, BytesMut, BigEndian};
use kraken_smtp::{ServerReply, ServerReplyCode};
use tokio_io::codec::Encoder;
use std::io::{Error, Result};

pub struct KrakenSmtpEncoder;

impl Encoder for KrakenSmtpEncoder {
    type Item = ServerReply;
    type Error = Error;

    fn encode(&mut self, server_reply: ServerReply, buf: &mut BytesMut) -> Result<()> {
        let line_count = server_reply.reply_lines.len();

        for line in server_reply.reply_lines[0..(line_count - 2)].iter() {
            let current_line = format!("{}-{}\r\n", server_reply.reply_code, line);

            buf.extend(current_line.as_bytes());
        }

        if line_count > 0 {
            let last_line = format!("{} {}\r\n", server_reply.reply_code, server_reply.reply_lines[line_count - 1]);

            buf.extend(last_line.as_bytes());
        }

        Ok(())
    }
}

