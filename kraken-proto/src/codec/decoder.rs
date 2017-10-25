use bytes::{BufMut, BytesMut};
use tokio_io::codec::{Decoder};
use std::io::{Error, Result};
use kraken_smtp::ClientCommand;

pub struct KrakenSmtpDecoder;

impl Decoder for KrakenSmtpDecoder {
    type Item = ClientCommand;

    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<ClientCommand>> {
        Ok(None)
    }

}
