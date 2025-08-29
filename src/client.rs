use crate::config::Config;
use reqwest::Client;
use anyhow::Result;

pub struct ZivyObrazClient {
    client: Client,
    config: Config,
}

impl ZivyObrazClient {
    pub fn new(config: Config) -> Self {
        let client = Client::new();
        Self { client, config }
    }

    pub async fn send_cpu_temp(&self, cpu_temp: f32) -> Result<()> {
        let cpu_temp_str = cpu_temp.to_string();
        let params = vec![
            ("import_key", self.config.import_key.as_str()),
            ("cpu_temp", cpu_temp_str.as_str()),
        ];


        let response = self.client
            .get(&self.config.server_url)
            .query(&params)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("HTTP error {}: {}", status, text))
        }
    }
}
