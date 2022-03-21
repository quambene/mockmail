use async_trait::async_trait;
use mockmail::{send_mock_email, send_real_email, Email, MockClient, RealClient};

#[derive(Clone)]
pub struct EmailClient<T: std::marker::Send + std::marker::Sync> {
    pub client: T,
}

impl EmailClient<RealClient> {
    pub fn new() -> Result<EmailClient<RealClient>, anyhow::Error> {
        let client = RealClient::new()?;
        Ok(EmailClient { client })
    }
}

impl EmailClient<MockClient> {
    pub fn new() -> Result<EmailClient<MockClient>, anyhow::Error> {
        Ok(EmailClient { client: MockClient })
    }
}

#[async_trait]
pub trait SendEmail {
    async fn send(&self, email: Email) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl SendEmail for RealClient {
    async fn send(&self, email: Email) -> Result<(), anyhow::Error> {
        send_real_email(&self.ses_client, email).await
    }
}

#[async_trait]
impl SendEmail for MockClient {
    async fn send(&self, email: Email) -> Result<(), anyhow::Error> {
        send_mock_email(email).await
    }
}

#[async_trait]
impl<T> SendEmail for EmailClient<T>
where
    T: SendEmail + std::marker::Sync + std::marker::Send,
{
    async fn send(&self, email: Email) -> Result<(), anyhow::Error> {
        let _res = self.client.send(email).await;
        Ok(())
    }
}

pub async fn email_service(client: &impl SendEmail) -> Result<(), anyhow::Error> {
    let email = Email::default();
    client.send(email).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_email_service() {
        let client = EmailClient::<MockClient>::new().unwrap();

        let res = email_service(&client).await;

        assert!(res.is_ok())
    }
}
