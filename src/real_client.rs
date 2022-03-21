use log::info;
use rusoto_core::{credential::EnvironmentProvider, HttpClient, Region};
use rusoto_ses::SesClient;

#[derive(Clone)]
pub struct RealClient {
    pub ses_client: SesClient,
}

impl RealClient {
    pub fn new() -> Result<Self, anyhow::Error> {
        info!("Setting up email client");

        let region = Region::EuWest1;
        let http_client = HttpClient::new()?;
        let provider = EnvironmentProvider::default();
        let ses_client = SesClient::new_with(http_client, provider, region);

        Ok(Self { ses_client })
    }
}
