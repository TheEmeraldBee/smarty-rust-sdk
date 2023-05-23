use crate::sdk::authentication::{Authenticate, SecretKeyCredential};

/// Options that can be passed into a new client
/// num_retries: the number of retries that the client with run before giving up.
/// logging_enabled: whether we should send logging data
/// headers: Custom headers that you can pass in
/// authentication: A authentication for Smarty
pub struct Options {
    pub license: String,

    // Retry Sender
    pub num_retries: u32,

    // Logger
    pub logging_enabled: bool,

    // Custom Headers
    pub headers: Vec<(String, String)>,

    // Authentication
    pub authentication: Box<dyn Authenticate>
}

impl Default for Options {
    fn default() -> Self {
        Self {
            license: String::default(),
            num_retries: u32::default(),
            logging_enabled: false,
            headers: vec![],
            authentication: Box::new(SecretKeyCredential { auth_id: "".to_string(), auth_token: "".to_string() })
        }
    }
}
