use super::config::ServerConfig;

pub struct AuthService {

}

impl<'a> From<&'a ServerConfig> for AuthService {

    fn from(config: &'a ServerConfig) -> Self {
        AuthService {}
    }
}
