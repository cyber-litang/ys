use crate::config::{get_config, Mail};
use anyhow::{Context, Result};
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

pub async fn send_self_mail(subject: &str, text: &str) -> Result<()> {
    let Mail {
        domain,
        user,
        password,
        ..
    } = &get_config().mail;

    let email = Message::builder()
        .from(user.parse().context("Failed to parse user")?)
        .to(user.parse().context("Failed to parse to")?)
        .subject(subject)
        .body(text.to_string())?;
    let creds = Credentials::new(user.clone(), password.clone());
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(domain)?
            .credentials(creds)
            .build();

    mailer.send(email).await?;

    Ok(())
}
