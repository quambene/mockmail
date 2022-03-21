#[derive(Clone)]
pub struct MockClient;

impl MockClient {
    pub fn new() -> Result<MockClient, anyhow::Error> {
        Ok(MockClient)
    }
}
