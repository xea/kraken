extern crate bytes;
extern crate kraken_proto;
extern crate kraken_smtp;
extern crate tokio_io;

use bytes::{BufMut, BytesMut};
use kraken_smtp::ClientCommand;
use kraken_proto::codec::SmtpServerCodec;
use tokio_io::codec::{Decoder, Encoder};
use std::io::Error;

mod server {
    use super::*;

    use super::test_server_decode;

    #[test]
    fn all_common_newline_variations_are_recognised() {
    }

    #[test]
    fn plain_quit_commands_are_accepted_as_quit() {
        test_server_decode("quit\r\n", ClientCommand::Quit);
    }

}

fn test_server_decode(client_input: &str, expected: ClientCommand) {
    let mut codec = SmtpServerCodec;
    let mut buffer = BytesMut::with_capacity(512);

    buffer.put(client_input);

    match codec.decode(&mut buffer) {
        Ok(Some(result)) => assert_eq!(result, expected),
        e => assert!(false, format!("{:?}", e))
    }

    /*
    match (expected, codec.decode(&mut buffer)) {
        (Ok(Some(exp_val)), Ok(Some(act_val))) => assert!(true),
        (Ok(None), _) => assert!(false),
        _ => assert!(false)
//        result => assert_eq!(result, expected)
    }
    */
}
