use std::io::Error;
use std::fmt;
use self::address::AddressSpec;
use self::message::Message;

mod address;
mod message;

pub enum ClientCommand {
    Ehlo(EhloDomain),
    MailFrom(AddressSpec),
    ReceiptTo(AddressSpec),
    Data(Message),
    BinaryData(Message),
    Reset,
    NoOp,
    Quit
}

pub struct EhloDomain {
    domain: String
}

impl EhloDomain {
    pub fn new(domain: String) -> Result<EhloDomain, Error> {
        Ok(EhloDomain { domain: domain })
    }
}

pub struct ServerReply {
    pub reply_code: ServerReplyCode,
    pub reply_lines: Vec<String>
}

pub enum ServerReplyCode {
    GenericOk
}

impl ServerReplyCode {
    pub fn reply_code(&self) -> usize {
        match *self {
            ServerReplyCode::GenericOk => 250
        }
    }
}

impl fmt::Display for ServerReplyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reply_code())
    }
}
