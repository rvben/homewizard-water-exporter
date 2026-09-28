# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.8](https://github.com/rvben/homewizard-water-exporter/compare/v0.1.7...v0.1.8) - 2026-09-28

### Fixed

- **deps**: update rustls to 0.23.45 for RUSTSEC-2026-0285 ([69c5ccd](https://github.com/rvben/homewizard-water-exporter/commit/69c5ccdcb7360325cfaff99fd46a3f89c1f06fc3))

## [0.1.5] - 2025-01-23

### Added
- Dependabot configuration for automated dependency updates
- Enhanced Cargo.toml metadata for better crates.io discoverability

### Fixed
- Musl toolchain installation in release workflow for binary builds
- GitHub release creation with proper binary artifacts

## [0.1.4] - 2025-01-22

### Added
- OCI labels to Dockerfile for GitHub Container Registry integration
- Make release target for automated release process
- Multi-platform Docker builds (linux/amd64, linux/arm64, linux/arm/v7)

### Changed
- Standardized user naming in Docker container to 'exporter'
- Updated Docker build to use musl-based Alpine for better portability
- Improved release workflow to commit Cargo.lock automatically

### Fixed
- Cargo.lock commit issues in release pipeline
- Docker architecture mismatch in builds

## [0.1.3] - 2025-01-15

### Added
- Initial Prometheus exporter for HomeWizard Water Meter
- Real-time water consumption monitoring
- Support for flow rate and total consumption metrics
- Health check endpoint
- Docker support with multi-stage builds
- GitHub Actions CI/CD pipeline

### Features
- Water meter integration with HomeWizard API
- Current flow rate metrics
- Total water consumption tracking
- Device status monitoring
- Configurable polling intervals
- TLS-enabled HTTP client
