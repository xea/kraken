extern crate bytes;
extern crate kraken_proto;
extern crate kraken_smtp;
extern crate tokio_io;

use bytes::BytesMut;
use kraken_smtp::{ServerReply, ServerReplyCode, ClientCommand, EhloDomain};
use tokio_io::codec::{Decoder, Encoder};
use kraken_proto::codec::{SmtpServerCodec, SmtpClientCodec};

mod server {

    use kraken_smtp::*;

    use super::test_server_encode;

    #[test]
    fn encoded_single_line_server_replies_dont_include_dash_between_reply_code_and_message() {
        test_server_encode(ServerReply { reply_code: ServerReplyCode::GenericOk, reply_lines: vec![ String::from("Test message") ] }, &b"250 Test message\r\n"[..]);
    }

    #[test]
    fn encoded_multi_line_server_replies_are_dash_separated_except_the_last_line() {
        let reply = ServerReply { reply_code: ServerReplyCode::GenericOk, reply_lines: vec![ String::from("A"), String::from("B") ] };
        test_server_encode(reply, &b"250-A\r\n250 B\r\n"[..]);
    }

    #[test]
    fn generic_ok_replies_are_encoded_with_code_250() {
        let reply = ServerReply { reply_code: ServerReplyCode::GenericOk, reply_lines: vec![] };
        test_server_encode(reply, &b"250\r\n"[..]);
    }

    #[test]
    fn server_replies_without_messages_return_the_reply_code_only() {
        test_server_encode(ServerReply { reply_code: ServerReplyCode::GenericOk, reply_lines: vec![] }, &b"250\r\n"[..]);
    }
}

mod client {
    use kraken_smtp::*;

    use super::test_client_encode;
}

fn test_server_encode(server_reply: ServerReply, expected: &[u8]) {
    let mut codec = SmtpServerCodec;
    let mut buffer = BytesMut::with_capacity(512);

    let _ = codec.encode(server_reply, &mut buffer);

    let read = buffer.take();

    assert_eq!(read, expected);
}

fn test_client_encode(command: ClientCommand, expected: &[u8]) {
    let mut codec = SmtpClientCodec;
    let mut buffer = BytesMut::with_capacity(512);

    let _ = codec.encode(command, &mut buffer);

    let read = buffer.take();

    assert_eq!(read, expected);
}
