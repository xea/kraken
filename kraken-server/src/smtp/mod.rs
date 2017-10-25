use super::config::ServerConfig;

pub struct SmtpService;

impl<'a> From<&'a ServerConfig> for SmtpService {

    fn from(config: &'a ServerConfig) -> Self {
        SmtpService
    }
}

impl SmtpService {

}
