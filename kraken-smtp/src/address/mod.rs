#[derive(Debug, PartialEq)]
pub enum Address {
    MailBox(MailBox),
    AddressGroup(AddressGroup)
}


#[derive(Debug, PartialEq)]
pub enum MailBox {
    NameAddress(NameAddress),
    AddressSpec(AddressSpec)
}

#[derive(Debug, PartialEq)]
pub struct AddressGroup {
    display_name: String,
    mailboxes: Vec<MailBox>
}

impl AddressGroup {
    pub fn new(display_name: String, mailboxes: Vec<MailBox>) -> Self {
        AddressGroup {
            display_name: display_name,
            mailboxes: mailboxes
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NameAddress {
    display_name: Option<String>,
    address: AngleAddress
}

#[derive(Debug, PartialEq)]
pub struct AngleAddress {
    spec: AddressSpec
}

#[derive(Debug, PartialEq)]
pub struct AddressSpec {
    local_part: String,
    domain: DomainSpec
}

// TODO this class doesn't actually follow the RFC
#[derive(Debug, PartialEq)]
pub struct DomainSpec {
    domain_name: String
}
