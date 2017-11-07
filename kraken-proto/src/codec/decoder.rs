use bytes::{BufMut, BytesMut};
use tokio_io::codec::{Decoder};
use std::io::{Error, ErrorKind, Result};
use kraken_smtp::{ ClientCommand, EhloDomain, ServerReply, ServerReplyCode };
use kraken_smtp::address::{ EnvelopeAddress, AddressSpec, DomainSpec };
use super::{SmtpServerCodec, SmtpClientCodec };

impl Decoder for SmtpServerCodec {

    type Item = ClientCommand;

    type Error = Error;

    /// Decode a raw message sent by an SMTP client
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<ClientCommand>> {
        const MAX_LINE_LENGTH: usize = 4096;

        if let Some(nl_idx) = buf.iter().position(|&b| b == b'\n') {
            let (last_idx, remaining) = if nl_idx > 0 && buf[nl_idx - 1] == b'\r' {
                (nl_idx - 1, 2)
            } else {
                (nl_idx, 1)
            };

            let unprocessed_line = buf.split_to(last_idx);
            // Throw away unnecessary line endings
            buf.split_to(remaining);

            let processed_line = ::std::str::from_utf8(&unprocessed_line)
                .map(|input| input.trim())
                .map(|input| String::from(input))
                .map(|input| self.parse_smtp_command(input))
                .ok();

            Ok(processed_line)

        } else {
            if buf.len() >= MAX_LINE_LENGTH {
                let _ = buf.split_to(MAX_LINE_LENGTH);

                Ok(Some(ClientCommand::LineTooLong))
            } else {
                Ok(None)
            }
        }
    }
}

impl SmtpServerCodec {
    fn parse_smtp_command(&mut self, input: String) -> ClientCommand {
        let uc_input = input.to_uppercase();

        if uc_input.starts_with("EHLO") {
            self.parse_ehlo(input)
        } else if uc_input.starts_with("MAIL FROM") {
            self.parse_mail_from(input)
        } else if uc_input.starts_with("RCPT TO") {
            self.parse_recipient_to(input)
        } else if uc_input.starts_with("QUIT ") {
            ClientCommand::Quit
        } else {
            ClientCommand::UnknownCommand(input)
        }
    }

    fn parse_ehlo(&mut self, input: String) -> ClientCommand {
        input.splitn(2, " ")
            .nth(1)
            .map(|ehloname| ClientCommand::Ehlo(EhloDomain { domain: String::from(ehloname) }) )
            .unwrap_or(ClientCommand::UnknownCommand(input))
    }

    fn parse_mail_from(&mut self, input: String) -> ClientCommand {
        input.splitn(2, ":")
            .nth(1)
            .map(|sender| {
                match self.parse_envelope_address(sender) {
                    Some(address) => ClientCommand::MailFrom(address),
                    None => ClientCommand::LineTooLong
                }})
            .unwrap_or(ClientCommand::UnknownCommand(input))
    }

    // TODO as this function is essentially identical to `parse_mail_from` it would be nice to
    // dedeuplicate them at some point
    fn parse_recipient_to(&mut self, input: String) -> ClientCommand {
        input.splitn(2, ":")
            .nth(1)
            .map(|recipient| {
                match self.parse_envelope_address(recipient) {
                    Some(address) => ClientCommand::ReceiptTo(address),
                    None => ClientCommand::LineTooLong
                }})
            .unwrap_or(ClientCommand::UnknownCommand(input))
    }

    fn parse_envelope_address(&mut self, address: &str) -> Option<EnvelopeAddress> {
        if "<>" == address.trim() {
            Some(EnvelopeAddress::NullAddress)
        } else {
            let address_split: Vec<&str> = address.trim()
                                    .trim_matches(|c| c == '<' || c == '>')
                                    .splitn(2, "@")
                                    .collect();

            if address_split.len() > 1 {
                let local_part = String::from(address_split[0]);
                let domain_spec = DomainSpec { domain_name: String::from(address_split[1]) };

                let address = EnvelopeAddress::AddressSpec(AddressSpec { local_part: local_part, domain: domain_spec });
                Some(address)
            } else {
                None
            }
        }
    }
}

impl Decoder for SmtpClientCodec {

    type Item = ServerReply;

    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<ServerReply>> {
        if let Some(_) = buf.iter().position(|&b| b == b'\n') {
            Ok(None)
        } else {
            Ok(None)
        }
        //Ok(Some(ServerReply { reply_code: ServerReplyCode::GenericOk, reply_lines: vec![] }))
    }

}
