use async_trait::async_trait;
use mockmail::{send_mock_email, send_real_email, Email, MockClient, RealClient};

#[derive(Clone)]
pub enum EmailClient {
    Real(RealClient),
    #[cfg(test)]
    Mock(MockClient),
}

impl EmailClient {
    pub fn new() -> Result<Self, anyhow::Error> {
        let real_client = RealClient::new()?;
        Ok(Self::Real(real_client))
    }

    #[cfg(test)]
    pub fn mock() -> Result<Self, anyhow::Error> {
        let mock_client = MockClient::new()?;
        Ok(Self::Mock(mock_client))
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
impl SendEmail for EmailClient {
    async fn send(&self, email: Email) -> Result<(), anyhow::Error> {
        match self {
            EmailClient::Real(real_client) => real_client.send(email).await,
            #[cfg(test)]
            EmailClient::Mock(mock_client) => mock_client.send(email).await,
        }
    }
}

pub async fn email_service(client: &EmailClient) -> Result<(), anyhow::Error> {
    let email = Email::default();
    client.send(email).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_email_service() {
        let client = EmailClient::mock().unwrap();

        let res = email_service(&client).await;

        assert!(res.is_ok())
    }
}
