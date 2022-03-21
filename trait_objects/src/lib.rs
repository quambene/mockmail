pub use async_trait::async_trait;
pub use mockmail::{send_mock_email, send_real_email, Email, MockClient, RealClient};

#[async_trait]
pub trait SendEmail {
    async fn send(&self, email: Email) -> Result<(), anyhow::Error>;
}

impl dyn SendEmail {
    pub fn new() -> Result<RealClient, anyhow::Error> {
        RealClient::new()
    }

    pub fn mock() -> Result<MockClient, anyhow::Error> {
        MockClient::new()
    }
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

pub async fn email_service(client: &dyn SendEmail) -> Result<(), anyhow::Error> {
    let email = Email::default();
    client.send(email).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_email_service() {
        let client = <dyn SendEmail>::mock().unwrap();

        let res = email_service(&client).await;

        assert!(res.is_ok())
    }
}
