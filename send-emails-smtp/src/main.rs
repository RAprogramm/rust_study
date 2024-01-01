mod config;
mod email;

use dotenv::dotenv;
use email::Email;
use serde::Serialize;

/// Represents user information including name and email.
#[derive(Debug, Serialize)]
pub struct User {
    /// User's name.
    pub name: String,
    /// User's email address.
    pub email: String,
}

#[tokio::main]
async fn main() {
    // Load Environment Variables
    dotenv().ok();

    let config = config::Config::init();

    // Create a User instance
    let user = User {
        name: String::from("RAprogramm"),
        email: config.smtp_to.to_owned(),
    };

    let verification_code = "my_ultra_secure_verification_code";
    let verification_url = format!("http://localhost:3000/verifyemail/{}", verification_code);

    //  Create an Email instance
    let email = Email::new(user, verification_url, config);

    // Send a verification code email
    if let Err(err) = email.send_verification_code().await {
        eprintln!("Failed to send verification code email: {:?}", err);
    } else {
        println!("Email verification code sent successfully!");
    }

    // Send a password reset token email
    if let Err(err) = email.send_password_reset_token().await {
        eprintln!("Failed to send password reset token email: {:?}", err);
    } else {
        println!("Password reset token email sent successfully!");
    }
}
