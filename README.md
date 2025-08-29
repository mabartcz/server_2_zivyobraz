# Živý obraz System Monitor

A simple Rust application that monitors system metrics (CPU temperature, RAM usage, disk usage, CPU usage) and sends them to the Živý obraz server.

## Features

- 🌡️ CPU temperature monitoring
- 💾 RAM usage monitoring
- 💿 Disk usage monitoring
- ⚡ CPU usage monitoring
- ⚙️ Configurable via `.env` file
- 🔄 Automatic periodic data sending
- 🛡️ HTTPS support for secure data transmission

## Setup

1. **Copy the example environment file:**
   ```bash
   cp .env.example .env
   ```

2. **Edit the `.env` file with your configuration:**
   ```bash
   # Your unique import key from Živý obraz
   IMPORT_KEY=your_unique_import_key_here
   
   # Server URL (use HTTPS for security)
   SERVER_URL=https://in.zivyobraz.eu
   
   # How often to send data (in minutes, minimum 1)
   SEND_INTERVAL_MINUTES=1
   
   # Enable/disable specific monitors
   ENABLE_CPU_TEMP=true
   ENABLE_RAM_USAGE=true
   ENABLE_DISK_USAGE=true
   ENABLE_CPU_USAGE=true
   
   # Variable names as they will appear in Živý obraz
   CPU_TEMP_VAR=cpu_teplota
   RAM_USAGE_VAR=ram_pouziti
   DISK_USAGE_VAR=disk_pouziti
   CPU_USAGE_VAR=cpu_pouziti
   ```

3. **Get your import key:**
   - Go to your Živý obraz dashboard
   - Navigate to the "Hodnoty" (Values) section
   - Copy your unique `import_key`
   - Paste it into the `.env` file

## Running the Application

### Development Mode
```bash
cargo run
```

### Release Mode (Optimized)
```bash
cargo build --release
./target/release/server_2_zivyobraz
```

### Running in Background (Linux)
```bash
nohup ./target/release/server_2_zivyobraz > monitor.log 2>&1 &
```

## Configuration Options

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `IMPORT_KEY` | Your unique Živý obraz import key | - | ✅ |
| `SERVER_URL` | Živý obraz server URL | `https://in.zivyobraz.eu` | ❌ |
| `SEND_INTERVAL_MINUTES` | Data sending interval in minutes | `1` | ❌ |
| `ENABLE_CPU_TEMP` | Enable CPU temperature monitoring | `true` | ❌ |
| `ENABLE_RAM_USAGE` | Enable RAM usage monitoring | `true` | ❌ |
| `ENABLE_DISK_USAGE` | Enable disk usage monitoring | `true` | ❌ |
| `ENABLE_CPU_USAGE` | Enable CPU usage monitoring | `true` | ❌ |
| `CPU_TEMP_VAR` | Variable name for CPU temperature | `cpu_teplota` | ❌ |
| `RAM_USAGE_VAR` | Variable name for RAM usage | `ram_pouziti` | ❌ |
| `DISK_USAGE_VAR` | Variable name for disk usage | `disk_pouziti` | ❌ |
| `CPU_USAGE_VAR` | Variable name for CPU usage | `cpu_pouziti` | ❌ |

## Data Format

The application sends data as HTTP GET requests with the following format:
```
https://in.zivyobraz.eu/?import_key=YOUR_KEY&cpu_teplota=45.2&ram_pouziti=67.8&disk_pouziti=34.5&cpu_pouziti=12.3
```

## Output Example

```
🚀 Starting Živý obraz system monitor
📋 Configuration loaded:
  Server: https://in.zivyobraz.eu
  Send interval: 1 minutes
  Enabled monitors:
    - CPU temperature
    - RAM usage
    - Disk usage
    - CPU usage

🔄 Starting monitoring loop...

Sending data to Živý obraz:
  cpu_teplota: 45.2
  ram_pouziti: 67.8
  disk_pouziti: 34.5
  cpu_pouziti: 12.3
✓ Data sent successfully
⏰ Waiting 1 minutes until next send...
```

## Important Notes

- **Minimum interval:** Don't send data more frequently than once per minute as per Živý obraz guidelines
- **CPU Temperature:** May not be available on all systems; the application will skip it if not detected
- **Multiple values:** If you need to send more than ~15 values, consider splitting them into multiple requests
- **Error handling:** The application will continue running even if individual requests fail

## Building for Different Platforms

### Linux (Cross-compile from Windows)
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

### Windows
```bash
cargo build --release
```

## Troubleshooting

1. **"IMPORT_KEY must be set" error:**
   - Make sure you have a `.env` file in the project root
   - Verify your import key is correctly set in the `.env` file

2. **No CPU temperature data:**
   - CPU temperature sensors may not be available on your system
   - The application will continue without temperature data

3. **HTTP errors:**
   - Check your internet connection
   - Verify your import key is correct
   - Ensure the server URL is accessible

## License

This project is provided as-is for use with Živý obraz systems.
