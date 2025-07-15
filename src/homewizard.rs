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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_homewizard_client_creation() {
        let client = HomeWizardClient::new(
            "http://192.168.1.100/api/v1/data".to_string(),
            Duration::from_secs(5),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_homewizard_client_creation_with_different_timeout() {
        let client = HomeWizardClient::new(
            "http://192.168.1.100/api/v1/data".to_string(),
            Duration::from_secs(30),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_homewizard_error_display() {
        let error = HomeWizardError::ParseError("Invalid JSON".to_string());
        assert_eq!(error.to_string(), "Failed to parse response: Invalid JSON");
    }

    #[test]
    fn test_homewizard_water_data_deserialization() {
        let json_data = r#"
        {
            "wifi_ssid": "HomeNetwork",
            "wifi_strength": 75.5,
            "total_liter_m3": 1234.567,
            "active_liter_lpm": 15.5,
            "total_liter_offset_m3": 100.0
        }
        "#;

        let data: Result<HomeWizardWaterData, _> = serde_json::from_str(json_data);
        assert!(data.is_ok());

        let data = data.unwrap();
        assert_eq!(data.wifi_ssid, "HomeNetwork");
        assert_eq!(data.wifi_strength, 75.5);
        assert_eq!(data.total_liter_m3, 1234.567);
        assert_eq!(data.active_liter_lpm, 15.5);
        assert_eq!(data.total_liter_offset_m3, 100.0);
    }

    #[test]
    fn test_homewizard_water_data_deserialization_minimal() {
        let json_data = r#"
        {
            "wifi_ssid": "Test",
            "wifi_strength": 50.0,
            "total_liter_m3": 100.0,
            "active_liter_lpm": 0.0,
            "total_liter_offset_m3": 0.0
        }
        "#;

        let data: Result<HomeWizardWaterData, _> = serde_json::from_str(json_data);
        assert!(data.is_ok());

        let data = data.unwrap();
        assert_eq!(data.wifi_ssid, "Test");
        assert_eq!(data.wifi_strength, 50.0);
        assert_eq!(data.total_liter_m3, 100.0);
        assert_eq!(data.active_liter_lpm, 0.0);
        assert_eq!(data.total_liter_offset_m3, 0.0);
    }

    #[test]
    fn test_homewizard_water_data_clone() {
        let data = HomeWizardWaterData {
            wifi_ssid: "Test".to_string(),
            wifi_strength: 50.0,
            total_liter_m3: 100.0,
            active_liter_lpm: 5.0,
            total_liter_offset_m3: 10.0,
        };

        let cloned = data.clone();
        assert_eq!(data.wifi_ssid, cloned.wifi_ssid);
        assert_eq!(data.wifi_strength, cloned.wifi_strength);
        assert_eq!(data.total_liter_m3, cloned.total_liter_m3);
        assert_eq!(data.active_liter_lpm, cloned.active_liter_lpm);
        assert_eq!(data.total_liter_offset_m3, cloned.total_liter_offset_m3);
    }

    #[test]
    fn test_homewizard_water_data_with_high_values() {
        let json_data = r#"
        {
            "wifi_ssid": "HighUsage",
            "wifi_strength": 100.0,
            "total_liter_m3": 9999.999,
            "active_liter_lpm": 999.0,
            "total_liter_offset_m3": 500.0
        }
        "#;

        let data: Result<HomeWizardWaterData, _> = serde_json::from_str(json_data);
        assert!(data.is_ok());

        let data = data.unwrap();
        assert_eq!(data.wifi_ssid, "HighUsage");
        assert_eq!(data.wifi_strength, 100.0);
        assert_eq!(data.total_liter_m3, 9999.999);
        assert_eq!(data.active_liter_lpm, 999.0);
        assert_eq!(data.total_liter_offset_m3, 500.0);
    }

    #[test]
    fn test_homewizard_water_data_with_zero_values() {
        let json_data = r#"
        {
            "wifi_ssid": "ZeroTest",
            "wifi_strength": 0.0,
            "total_liter_m3": 0.0,
            "active_liter_lpm": 0.0,
            "total_liter_offset_m3": 0.0
        }
        "#;

        let data: Result<HomeWizardWaterData, _> = serde_json::from_str(json_data);
        assert!(data.is_ok());

        let data = data.unwrap();
        assert_eq!(data.wifi_ssid, "ZeroTest");
        assert_eq!(data.wifi_strength, 0.0);
        assert_eq!(data.total_liter_m3, 0.0);
        assert_eq!(data.active_liter_lpm, 0.0);
        assert_eq!(data.total_liter_offset_m3, 0.0);
    }

    #[test]
    fn test_homewizard_error_from_reqwest() {
        // Create a reqwest error by making a request to an invalid URL
        let client = reqwest::Client::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async {
            let result = client
                .get("http://invalid-url-that-does-not-exist.test")
                .send()
                .await;
            assert!(result.is_err());

            let reqwest_error = result.unwrap_err();
            let hw_error = HomeWizardError::from(reqwest_error);

            match hw_error {
                HomeWizardError::RequestFailed(_) => {
                    // This is expected
                }
                _ => panic!("Expected RequestFailed error"),
            }
        });
    }

    #[tokio::test]
    async fn test_fetch_data_success() {
        let mock_server = MockServer::start().await;
        let json_response = r#"
        {
            "wifi_ssid": "TestNetwork",
            "wifi_strength": 75.5,
            "total_liter_m3": 1234.567,
            "active_liter_lpm": 15.5,
            "total_liter_offset_m3": 100.0
        }
        "#;

        Mock::given(method("GET"))
            .and(path("/api/v1/data"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::from_str::<serde_json::Value>(json_response).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let client = HomeWizardClient::new(
            format!("{}/api/v1/data", mock_server.uri()),
            Duration::from_secs(5),
        )
        .unwrap();

        let result = client.fetch_data().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.wifi_ssid, "TestNetwork");
        assert_eq!(data.wifi_strength, 75.5);
        assert_eq!(data.total_liter_m3, 1234.567);
        assert_eq!(data.active_liter_lpm, 15.5);
        assert_eq!(data.total_liter_offset_m3, 100.0);
    }

    #[tokio::test]
    async fn test_fetch_data_http_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/data"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let client = HomeWizardClient::new(
            format!("{}/api/v1/data", mock_server.uri()),
            Duration::from_secs(5),
        )
        .unwrap();

        let result = client.fetch_data().await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HomeWizardError::ParseError(msg) => {
                assert!(msg.contains("HTTP status: 500"));
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[tokio::test]
    async fn test_fetch_data_malformed_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/data"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .mount(&mock_server)
            .await;

        let client = HomeWizardClient::new(
            format!("{}/api/v1/data", mock_server.uri()),
            Duration::from_secs(5),
        )
        .unwrap();

        let result = client.fetch_data().await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HomeWizardError::RequestFailed(_) => {
                // This is expected for JSON parsing errors
            }
            _ => panic!("Expected RequestFailed error"),
        }
    }

    #[tokio::test]
    async fn test_fetch_data_timeout() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/data"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("response")
                    .set_delay(Duration::from_millis(500)),
            )
            .mount(&mock_server)
            .await;

        let client = HomeWizardClient::new(
            format!("{}/api/v1/data", mock_server.uri()),
            Duration::from_millis(100), // Very short timeout
        )
        .unwrap();

        let result = client.fetch_data().await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HomeWizardError::RequestFailed(_) => {
                // This is expected for timeout errors
            }
            _ => panic!("Expected RequestFailed error"),
        }
    }

    #[tokio::test]
    async fn test_fetch_data_connection_refused() {
        // Use a port that's definitely not listening
        let client = HomeWizardClient::new(
            "http://127.0.0.1:12345/api/v1/data".to_string(),
            Duration::from_secs(5),
        )
        .unwrap();

        let result = client.fetch_data().await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HomeWizardError::RequestFailed(_) => {
                // This is expected for connection refused errors
            }
            _ => panic!("Expected RequestFailed error"),
        }
    }

    #[tokio::test]
    async fn test_fetch_data_missing_fields() {
        let mock_server = MockServer::start().await;
        let incomplete_json = r#"
        {
            "wifi_ssid": "TestNetwork",
            "wifi_strength": 75.5
        }
        "#;

        Mock::given(method("GET"))
            .and(path("/api/v1/data"))
            .respond_with(ResponseTemplate::new(200).set_body_string(incomplete_json))
            .mount(&mock_server)
            .await;

        let client = HomeWizardClient::new(
            format!("{}/api/v1/data", mock_server.uri()),
            Duration::from_secs(5),
        )
        .unwrap();

        let result = client.fetch_data().await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HomeWizardError::RequestFailed(_) => {
                // This is expected for missing fields
            }
            _ => panic!("Expected RequestFailed error"),
        }
    }

    #[tokio::test]
    async fn test_fetch_data_different_status_codes() {
        let mock_server = MockServer::start().await;

        // Test 404 Not Found
        Mock::given(method("GET"))
            .and(path("/api/v1/data"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = HomeWizardClient::new(
            format!("{}/api/v1/data", mock_server.uri()),
            Duration::from_secs(5),
        )
        .unwrap();

        let result = client.fetch_data().await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HomeWizardError::ParseError(msg) => {
                assert!(msg.contains("HTTP status: 404"));
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[tokio::test]
    async fn test_fetch_data_empty_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/data"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&mock_server)
            .await;

        let client = HomeWizardClient::new(
            format!("{}/api/v1/data", mock_server.uri()),
            Duration::from_secs(5),
        )
        .unwrap();

        let result = client.fetch_data().await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HomeWizardError::RequestFailed(_) => {
                // This is expected for empty responses
            }
            _ => panic!("Expected RequestFailed error"),
        }
    }
}
