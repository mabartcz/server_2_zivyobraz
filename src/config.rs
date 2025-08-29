use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub import_key: String,
    pub server_url: String,
    pub send_interval_minutes: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok(); // Load .env file if it exists

        let import_key = env::var("IMPORT_KEY")
            .map_err(|_| anyhow::anyhow!("IMPORT_KEY must be set in environment or .env file"))?;

        let server_url = env::var("SERVER_URL")
            .unwrap_or_else(|_| "https://in.zivyobraz.eu".to_string());

        let send_interval_minutes = env::var("SEND_INTERVAL_MINUTES")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<u64>()
            .unwrap_or(1);

        Ok(Config {
            import_key,
            server_url,
            send_interval_minutes,
        })
    }
}
