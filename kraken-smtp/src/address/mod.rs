pub enum Address {
    MailBox(MailBox),
    AddressGroup(AddressGroup)
}


pub enum MailBox {
    NameAddress(NameAddress),
    AddressSpec(AddressSpec)
}

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

pub struct NameAddress {
    display_name: Option<String>,
    address: AngleAddress
}

pub struct AngleAddress {
    spec: AddressSpec
}

pub struct AddressSpec {
    local_part: String,
    domain: DomainSpec
}

// TODO this class doesn't actually follow the RFC
pub struct DomainSpec {
    domain_name: String
}
