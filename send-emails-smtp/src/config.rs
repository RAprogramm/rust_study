use std::env::var;

/// Represents configuration details required for SMTP (Simple Mail Transfer Protocol) settings.
#[derive(Debug, Clone)]
pub struct Config {
    /// SMTP server host address.
    pub smtp_host: String,
    /// SMTP server port number.
    pub smtp_port: u16,
    /// Username for authentication with the SMTP server.
    pub smtp_user: String,
    /// Password for authentication with the SMTP server.
    pub smtp_pass: String,
    /// Sender's email address.
    pub smtp_from: String,
    /// Receiver's email address.
    pub smtp_to: String,
}

impl Config {
    /// Initializes and constructs a `Config` object based on environment variables.
    ///
    /// # Panics
    ///
    /// Panics if any of the required environment variables (`SMTP_HOST`, `SMTP_PORT`, `SMTP_USER`,
    /// `SMTP_PASS`, `SMTP_FROM`, `SMTP_TO`) are not set or if `SMTP_PORT` is not a valid `u16`.
    ///
    /// # Returns
    ///
    /// A `Config` object initialized with values from the environment variables.
    pub fn init() -> Config {
        let smtp_host = var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = var("SMTP_PORT").expect("SMTP_PORT must be set");
        let smtp_user = var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_pass = var("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_from = var("SMTP_FROM").expect("SMTP_FROM must be set");
        let smtp_to = var("SMTP_TO").expect("SMTP_TO must be set");

        Config {
            smtp_host,
            smtp_port: smtp_port.parse::<u16>().expect("Invalid SMTP_PORT"),
            smtp_user,
            smtp_pass,
            smtp_from,
            smtp_to,
        }
    }
}
