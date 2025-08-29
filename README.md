# Živý obraz System Monitor

A simple Rust application that monitors CPU temperature and sends it to the Živý obraz server.

## Setup

1. **Configure environment:**
   ```bash
   # Set your import key and server URL
   set IMPORT_KEY=your_key_here
   set SERVER_URL=https://in.zivyobraz.eu
   set SEND_INTERVAL_MINUTES=1
   ```

2. **Run:**
   ```bash
   cargo run
   ```

## What it does

- Monitors CPU package temperature using system sensors
- Sends temperature data to Živý obraz every minute
- Works on Windows with coretemp sensors
