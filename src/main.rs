mod config;
mod monitor;
mod client;

use config::Config;
use monitor::SystemMonitor;
use client::ZivyObrazClient;
use tokio::time::{sleep, Duration};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Starting Živý obraz CPU temperature monitor");

    // Load configuration
    let config = Config::from_env()?;
    println!("📋 Configuration loaded:");
    println!("  Server: {}", config.server_url);
    println!("  Send interval: {} minutes", config.send_interval_minutes);

    // Initialize system monitor and client
    let mut monitor = SystemMonitor::new();
    let client = ZivyObrazClient::new(config.clone());

    // Wait a moment for initial system data collection
    sleep(Duration::from_secs(2)).await;

    println!("\n🔄 Starting monitoring loop...\n");

    loop {
        // Get CPU temperature
        match monitor.get_cpu_temperature() {
            Some(cpu_temp) => {
                // Send CPU temperature to Živý obraz
                println!("🌡️ Current CPU temperature: {:.1}°C", cpu_temp);
                match client.send_cpu_temp(cpu_temp).await {
                    Ok(()) => {},
                    Err(e) => eprintln!("❌ Error sending data: {}", e),
                }
            },
            None => {
                eprintln!("⚠️ Could not read CPU temperature");
            }
        }

        println!("⏰ Waiting {} minutes until next send...\n", config.send_interval_minutes);

        // Wait for the configured interval
        sleep(Duration::from_secs(config.send_interval_minutes * 60)).await;
    }
}
