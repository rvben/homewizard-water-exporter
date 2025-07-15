# HomeWizard Water Prometheus Exporter

[![CI](https://github.com/rvben/homewizard-water-exporter/actions/workflows/ci.yml/badge.svg)](https://github.com/rvben/homewizard-water-exporter/actions/workflows/ci.yml)
[![Release](https://github.com/rvben/homewizard-water-exporter/actions/workflows/release.yml/badge.svg)](https://github.com/rvben/homewizard-water-exporter/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/homewizard-water-exporter.svg)](https://crates.io/crates/homewizard-water-exporter)
[![Docker Pulls](https://img.shields.io/docker/pulls/rvben/homewizard-water-exporter)](https://hub.docker.com/r/rvben/homewizard-water-exporter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-blue.svg)](https://www.rust-lang.org)

A Rust-based Prometheus exporter for the HomeWizard Water Meter, providing real-time water consumption metrics.

## Features

- 💧 **Real-time Monitoring** - Water consumption metrics updated every 60 seconds
- 🚰 **Flow Rate Tracking** - Active water flow measurement in liters per minute
- 📊 **Total Consumption** - Cumulative water usage tracking in cubic meters
- 📡 **Network Monitoring** - WiFi signal strength tracking
- 🚀 **High Performance** - Lightweight Rust implementation with minimal resource usage
- 🐳 **Docker Ready** - Multi-platform images for easy deployment
- ✅ **Production Ready** - Comprehensive test coverage and error handling
- 🔧 **Offset Support** - Handle meter replacements with offset tracking

## Prerequisites

- HomeWizard Water Meter with local API enabled
- Rust 1.88+ (for building from source)
- Docker (for container deployment)

## Quick Start

```bash
# Using Docker
docker run -d -p 9899:9899 -e HOMEWIZARD_HOST=192.168.1.241 rvben/homewizard-water-exporter:latest

# Or using pre-built binary
wget https://github.com/rvben/homewizard-water-exporter/releases/latest/download/homewizard-water-exporter-$(uname -m)-linux.tar.gz
tar -xzf homewizard-water-exporter-*.tar.gz
HOMEWIZARD_HOST=192.168.1.241 ./homewizard-water-exporter
```

## Installation

### Using Docker (Recommended)

```bash
# From Docker Hub
docker run -d \
  --name homewizard-water-exporter \
  -p 9899:9899 \
  -e HOMEWIZARD_HOST=192.168.1.241 \
  rvben/homewizard-water-exporter:latest

# From GitHub Container Registry
docker run -d \
  --name homewizard-water-exporter \
  -p 9899:9899 \
  -e HOMEWIZARD_HOST=192.168.1.241 \
  ghcr.io/rvben/homewizard-water-exporter:latest
```

### Using Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/rvben/homewizard-water-exporter/releases).

```bash
# Example for Linux x86_64
wget https://github.com/rvben/homewizard-water-exporter/releases/latest/download/homewizard-water-exporter-x86_64-linux.tar.gz
tar -xzf homewizard-water-exporter-x86_64-linux.tar.gz
chmod +x homewizard-water-exporter
HOMEWIZARD_HOST=192.168.1.241 ./homewizard-water-exporter
```

### Using Cargo

```bash
cargo install homewizard-water-exporter
HOMEWIZARD_HOST=192.168.1.241 homewizard-water-exporter
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/rvben/homewizard-water-exporter
cd homewizard-water-exporter

# Build the binary
cargo build --release

# Run the exporter
HOMEWIZARD_HOST=192.168.1.241 ./target/release/homewizard-water-exporter
```

## Configuration

The exporter can be configured via command-line arguments or environment variables:

| Environment Variable | CLI Flag | Default | Description |
|---------------------|----------|---------|-------------|
| `HOMEWIZARD_HOST` | `--host` | Required | IP address or hostname of HomeWizard Water Meter |
| `METRICS_PORT` | `--port` | `9899` | Port to expose Prometheus metrics |
| `POLL_INTERVAL` | `--poll-interval` | `60` | Seconds between API polls |
| `LOG_LEVEL` | `--log-level` | `info` | Log level (trace, debug, info, warn, error) |
| `HTTP_TIMEOUT` | `--http-timeout` | `5` | HTTP request timeout in seconds |

## Metrics

The exporter provides the following Prometheus metrics:

| Metric | Type | Description |
|--------|------|-------------|
| `homewizard_water_total_m3` | Counter | Total water consumption in m³ |
| `homewizard_water_active_flow_lpm` | Gauge | Current water flow in liters per minute |
| `homewizard_water_offset_m3` | Gauge | Water meter offset in m³ |
| `homewizard_water_wifi_strength_percent` | Gauge | WiFi signal strength percentage |
| `homewizard_water_meter_info{wifi_ssid}` | Gauge | Water meter information |

## Prometheus Configuration

Add the following to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'homewizard_water'
    static_configs:
      - targets: ['localhost:9899']
    scrape_interval: 60s
```

## Enabling HomeWizard Local API

1. Open the HomeWizard Energy app
2. Go to Settings → Meters → Your Water Meter
3. Enable "Local API"

## Example Metrics Output

```
# HELP homewizard_water_total_m3 Total water consumption in m³
# TYPE homewizard_water_total_m3 counter
homewizard_water_total_m3 451.827

# HELP homewizard_water_active_flow_lpm Current water flow in liters per minute
# TYPE homewizard_water_active_flow_lpm gauge
homewizard_water_active_flow_lpm 0

# HELP homewizard_water_wifi_strength_percent WiFi signal strength percentage
# TYPE homewizard_water_wifi_strength_percent gauge
homewizard_water_wifi_strength_percent 100
```

## Grafana Dashboard

An example Grafana dashboard is included in `grafana-dashboard.json`. To import:

1. Open Grafana
2. Go to Dashboards → Import
3. Upload the JSON file or paste its contents
4. Select your Prometheus data source
5. Click Import

The dashboard includes:
- Total water consumption
- Real-time flow rate graph
- Hourly water usage
- WiFi signal strength gauge
- Current flow rate display

## Development

```bash
# Show all available make targets
make help

# Build the binary
make build

# Run tests
make test

# Check code formatting and linting
make check

# Run the exporter locally
HOMEWIZARD_HOST=192.168.1.241 make run

# Build Docker image
make docker-build

# Run in Docker
HOMEWIZARD_HOST=192.168.1.241 make docker-run
```

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.