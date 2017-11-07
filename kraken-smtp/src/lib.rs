use std::io::Error;
use std::fmt;
use self::address::EnvelopeAddress;
use self::message::Message;

pub mod address;
pub mod message;

#[derive(Debug, PartialEq)]
pub enum ClientCommand {
    Ehlo(EhloDomain),
    MailFrom(EnvelopeAddress),
    ReceiptTo(EnvelopeAddress),
    DataBegin,
    BinaryDataBegin,
    Reset,
    NoOp,
    Quit,
    UnknownCommand(String),
    LineTooLong
}

#[derive(Debug, PartialEq)]
pub struct EhloDomain {
    pub domain: String
}

impl EhloDomain {
    pub fn new(domain: String) -> Result<EhloDomain, Error> {
        Ok(EhloDomain { domain: domain })
    }
}

#[derive(Debug)]
pub struct ServerReply {
    pub reply_code: ServerReplyCode,
    pub reply_lines: Vec<String>
}

#[derive(Debug, PartialEq)]
pub enum ServerReplyCode {
    // Normal responses
    ServerReady,
    ConnectionClosed,
    GenericOk,
    DataReady,
    // Rejections
    TemporaryRejection,
    // Errors
    SyntaxError,
    SyntaxParamError,
    NotImplemented,
    BadAddress,
    BadSequence
}

impl ServerReplyCode {
    pub fn reply_code(&self) -> usize {
        match *self {
            // Positive preliminary replies
            // Positive completion replies
            ServerReplyCode::ServerReady        => 220,
            ServerReplyCode::ConnectionClosed   => 221,
            ServerReplyCode::GenericOk          => 250,
            // Positive intermediate replies
            ServerReplyCode::DataReady          => 354,
            // Transient negative completion replies
            ServerReplyCode::TemporaryRejection => 451,
            // Permanent negative completion replies
            ServerReplyCode::SyntaxError        => 500,
            ServerReplyCode::SyntaxParamError   => 501,
            ServerReplyCode::NotImplemented     => 502,
            ServerReplyCode::BadSequence        => 503,
            ServerReplyCode::BadAddress         => 510,
        }
    }
}

impl fmt::Display for ServerReplyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reply_code())
    }
}
