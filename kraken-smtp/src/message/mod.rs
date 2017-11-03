#[derive(Debug, PartialEq)]
pub enum Message {
    RFC2822Message(RFC2822Message),
    RawMessage(RawMessage)
}

#[derive(Debug, PartialEq)]
pub struct RFC2822Message {
}

#[derive(Debug, PartialEq)]
pub struct RawMessage {
    //content: Vec<u8>
}
