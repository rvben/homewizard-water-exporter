use anyhow::Result;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HomeWizardError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct HomeWizardWaterData {
    pub wifi_ssid: String,
    pub wifi_strength: f64,
    pub total_liter_m3: f64,
    pub active_liter_lpm: f64,
    pub total_liter_offset_m3: f64,
}

pub struct HomeWizardClient {
    client: reqwest::Client,
    url: String,
}

impl HomeWizardClient {
    pub fn new(url: String, timeout: std::time::Duration) -> Result<Self> {
        let client = reqwest::Client::builder().timeout(timeout).build()?;

        Ok(Self { client, url })
    }

    pub async fn fetch_data(&self) -> Result<HomeWizardWaterData, HomeWizardError> {
        let response = self.client.get(&self.url).send().await?;

        if !response.status().is_success() {
            return Err(HomeWizardError::ParseError(format!(
                "HTTP status: {}",
                response.status()
            )));
        }

        let data = response.json::<HomeWizardWaterData>().await?;
        Ok(data)
    }
}
