use handlebars::Handlebars;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::{config::Config, User};

/// Represents an email to be sent to users for various actions.
pub struct Email {
    /// Represents the user associated with the email.
    user: User,
    /// Contains the URL relevant to the email context.
    url: String,
    /// Represents the sender's email address.
    from: String,
    /// Holds configurations related to SMTP (Simple Mail Transfer Protocol).
    config: Config,
}

impl Email {
    /// Creates a new `Email` instance with the provided user, URL, and configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_project::{Email, User, Config};
    ///
    /// // Assume these values are instantiated properly in your code.
    /// let user = User::new("John", "john@example.com");
    /// let url = String::from("https://example.com");
    /// let config = Config::load();
    ///
    /// // Create a new Email instance
    /// let email = Email::new(user, url, config);
    /// ```
    pub fn new(user: User, url: String, config: Config) -> Self {
        // Construct the sender's email address using the configured SMTP settings.
        let from = format!("RAprogramm <{}>", config.smtp_from.to_owned());

        Email {
            user,
            url,
            from,
            config,
        }
    }

    /// Creates a new SMTP transport for sending emails based on the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the SMTP transport configuration or setup fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_project::{Email, Config};
    ///
    /// // Assume Config is properly instantiated in your code.
    /// let config = Config::load();
    ///
    /// // Assume email_instance is an Email object with appropriate data.
    /// let email_instance = Email::new(/* user, url, and config */);
    ///
    /// // Create a new SMTP transport for sending emails
    /// let transport_result = email_instance.new_transport();
    ///
    /// // Handle the Result appropriately
    /// match transport_result {
    ///     Ok(transport) => {
    ///         // SMTP transport successfully created
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///         eprintln!("Error creating SMTP transport: {}", err);
    ///     }
    /// }
    /// ```
    fn new_transport(
        &self,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, lettre::transport::smtp::Error> {
        // Create credentials for authentication on the SMTP server
        let creds = Credentials::new(
            self.config.smtp_user.to_owned(),
            self.config.smtp_pass.to_owned(),
        );

        // Create an SMTP transport using TLS (starttls_relay) based on the configuration settings
        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
            &self.config.smtp_host.to_owned(),
        )?
        // Set the SMTP server port
        .port(self.config.smtp_port)
        // Pass credentials for authentication
        .credentials(creds)
        // Finalize the transport setup
        .build();

        // Return the result: Ok(transport) or Err in case of an error
        Ok(transport)
    }

    /// Renders an email template using Handlebars templates and user-specific data.
    ///
    /// # Arguments
    ///
    /// * `template_name` - The name of the template to render.
    ///
    /// # Returns
    ///
    /// A Result containing the rendered template string on success, or a RenderError on failure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_project::Email;
    ///
    /// // Assume 'email_instance' is an Email object with proper data.
    /// let email_instance = Email::new(/* user, url, and config */);
    ///
    /// // Render a template named 'verification_code'
    /// let rendered_template = email_instance.render_template("verification_code");
    ///
    /// // Handle the Result appropriately
    /// match rendered_template {
    ///     Ok(template) => {
    ///         // Use the rendered template
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///         eprintln!("Error rendering template: {}", err);
    ///     }
    /// }
    /// ```
    fn render_template(&self, template_name: &str) -> Result<String, handlebars::RenderError> {
        // Create a new Handlebars instance
        let mut handlebars = Handlebars::new();

        // Register the main template and necessary partials
        handlebars
            .register_template_file(template_name, &format!("./templates/{}.hbs", template_name))?;
        handlebars.register_template_file("styles", "./templates/partials/styles.hbs")?;
        handlebars.register_template_file("base", "./templates/partials/base.hbs")?;

        // Prepare data to be passed to the template
        let data = serde_json::json!({
            "first_name": &self.user.name.split_whitespace().next().unwrap_or_default(),
            "subject": &template_name,
            "url": &self.url
        });

        // Render the template using the prepared data
        let content_template = handlebars.render(template_name, &data)?;

        // Return the rendered template
        Ok(content_template)
    }

    /// Asynchronously sends an email using rendered HTML content from a template.
    ///
    /// # Arguments
    ///
    /// * `template_name` - The name of the template used for rendering the email content.
    /// * `subject` - The subject line of the email.
    ///
    /// # Returns
    ///
    /// A Result indicating success (()) on successful email transmission, or a Box<dyn Error>
    /// containing the error information in case of failure.
    ///
    /// # Errors
    ///
    /// Returns an error if rendering the template or sending the email fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_project::Email;
    ///
    /// // Assume 'email_instance' is an Email object with proper data.
    /// let email_instance = Email::new(/* user, url, and config */);
    ///
    /// // Send an email with a subject and content from the 'verification_code' template
    /// let send_result = email_instance.send_email("verification_code", "Subject Line").await;
    ///
    /// // Handle the Result appropriately
    /// match send_result {
    ///     Ok(()) => {
    ///         // Email sent successfully
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///         eprintln!("Error sending email: {}", err);
    ///     }
    /// }
    /// ```
    async fn send_email(
        &self,
        template_name: &str,
        subject: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Render the HTML content for the email using the specified template
        let html_template = self.render_template(template_name)?;

        // Build the email message
        let email = Message::builder()
            .to(
                format!("{} <{}>", self.user.name.as_str(), self.user.email.as_str())
                    .parse()
                    .unwrap(),
            )
            .reply_to(self.from.as_str().parse().unwrap())
            .from(self.from.as_str().parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_template)?;

        // Create a new SMTP transport and send the email asynchronously
        let transport = self.new_transport()?;
        transport.send(email).await?;

        // Return success if the email is sent successfully
        Ok(())
    }

    /// Sends an account verification code to the user's email.
    ///
    /// # Returns
    ///
    /// A Result indicating success (()) on successful email transmission, or a Box<dyn Error>
    /// containing the error information in case of failure.
    ///
    /// # Errors
    ///
    /// Returns an error if sending the verification code email fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_project::Email;
    ///
    /// // Assume 'email_instance' is an Email object with proper data.
    /// let email_instance = Email::new(/* user, url, and config */);
    ///
    /// // Send a verification code email
    /// let send_result = email_instance.send_verification_code().await;
    ///
    /// // Handle the Result appropriately
    /// match send_result {
    ///     Ok(()) => {
    ///         // Verification code email sent successfully
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///         eprintln!("Error sending verification code: {}", err);
    ///     }
    /// }
    /// ```
    pub async fn send_verification_code(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Send an email using the 'verification_code' template with the subject line
        // 'Your account verification code'
        self.send_email("verification_code", "Your account verification code")
            .await
    }

    /// Sends a password reset token to the user's email.
    ///
    /// # Returns
    ///
    /// A Result indicating success (()) on successful email transmission, or a Box<dyn Error>
    /// containing the error information in case of failure.
    ///
    /// # Errors
    ///
    /// Returns an error if sending the password reset token email fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_project::Email;
    ///
    /// // Assume 'email_instance' is an Email object with proper data.
    /// let email_instance = Email::new(/* user, url, and config */);
    ///
    /// // Send a password reset token email
    /// let send_result = email_instance.send_password_reset_token().await;
    ///
    /// // Handle the Result appropriately
    /// match send_result {
    ///     Ok(()) => {
    ///         // Password reset token email sent successfully
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///         eprintln!("Error sending password reset token: {}", err);
    ///     }
    /// }
    /// ```
    pub async fn send_password_reset_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Send an email using the 'reset_password' template with the subject line
        // 'Your password reset token (valid for only 10 minutes)'
        self.send_email(
            "reset_password",
            "Your password reset token (valid for only 10 minutes)",
        )
        .await
    }
}
