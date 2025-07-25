.PHONY: help build build-release run test lint fmt clean docker-build docker-buildx docker-push docker-push-ghcr docker-run release check gh-secrets coverage

# Default target
help:
	@echo "Available targets:"
	@echo "  make build        - Build the binary in debug mode"
	@echo "  make build-release - Build the binary in release mode"
	@echo "  make run          - Run the exporter (requires HOMEWIZARD_HOST)"
	@echo "  make test         - Run tests"
	@echo "  make coverage     - Generate code coverage report"
	@echo "  make lint         - Run clippy linter"
	@echo "  make fmt          - Format code"
	@echo "  make check        - Run format check and linter"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make docker-build - Build Docker image (local)"
	@echo "  make docker-buildx - Build multi-arch Docker image (local)"
	@echo "  make docker-push  - Build and push multi-arch to Docker Hub"
	@echo "  make docker-push-ghcr - Build and push multi-arch to GitHub Container Registry"
	@echo "  make docker-run   - Run Docker container (requires HOMEWIZARD_HOST)"
	@echo "  make release      - Prepare and create a release (requires VERSION=v0.1.0)"
	@echo "  make gh-secrets   - Set GitHub Actions secrets from .env file"

# Build debug binary
build:
	cargo build

# Build release binary
build-release:
	cargo build --release

# Run the exporter
run:
	@if [ -z "$$HOMEWIZARD_HOST" ]; then \
		echo "Error: HOMEWIZARD_HOST environment variable is required"; \
		echo "Usage: HOMEWIZARD_HOST=192.168.1.241 make run"; \
		exit 1; \
	fi
	cargo run

# Run tests
test:
	cargo test --verbose

# Generate code coverage report
coverage:
	cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out html

# Run linter
lint:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Check formatting and run linter
check:
	cargo fmt -- --check
	cargo clippy -- -D warnings

# Prepare a release
release:
	@if [ -z "$$VERSION" ]; then \
		echo "Error: VERSION environment variable is required"; \
		echo "Usage: VERSION=v0.1.0 make release"; \
		exit 1; \
	fi
	@echo "Preparing release $$VERSION..."
	@echo "1. Running checks..."
	@make check
	@echo "2. Updating version in Cargo.toml..."
	@VERSION_NUM=$$(echo $$VERSION | sed 's/^v//') && \
		sed -i.bak "s/^version = \".*\"/version = \"$$VERSION_NUM\"/" Cargo.toml && \
		rm Cargo.toml.bak
	@echo "3. Updating Cargo.lock..."
	@cargo update
	@echo "4. Committing version changes..."
	@git add Cargo.toml Cargo.lock
	@git commit -m "chore: bump version to $$VERSION" || echo "No changes to commit"
	@echo "5. Creating and pushing tag..."
	@git tag $$VERSION
	@git push origin $$VERSION
	@echo "Release $$VERSION created and pushed!"

# Clean build artifacts
clean:
	cargo clean

# Build Docker image
docker-build:
	docker build -t homewizard-water-exporter:latest .

# Build multi-arch Docker image (local)
docker-buildx:
	docker buildx build --platform linux/amd64,linux/arm64 -t homewizard-water-exporter .

# Build and push multi-arch Docker image to Docker Hub
docker-push:
	@if [ -z "$$DOCKER_USERNAME" ]; then \
		echo "Error: DOCKER_USERNAME environment variable is required"; \
		echo "Usage: DOCKER_USERNAME=youruser DOCKER_PASSWORD=yourpass make docker-push"; \
		exit 1; \
	fi
	@if [ -z "$$DOCKER_PASSWORD" ]; then \
		echo "Error: DOCKER_PASSWORD environment variable is required"; \
		echo "Usage: DOCKER_USERNAME=youruser DOCKER_PASSWORD=yourpass make docker-push"; \
		exit 1; \
	fi
	@echo "Logging in to Docker Hub..."
	@echo "$$DOCKER_PASSWORD" | docker login -u "$$DOCKER_USERNAME" --password-stdin
	@echo "Building and pushing multi-arch images..."
	docker buildx build --platform linux/amd64,linux/arm64 \
		-t $$DOCKER_USERNAME/homewizard-water-exporter:latest \
		-t $$DOCKER_USERNAME/homewizard-water-exporter:$$(git describe --tags --always) \
		--push .
	@echo "Successfully pushed to Docker Hub!"

# Build and push to GitHub Container Registry
docker-push-ghcr:
	@if [ -z "$$GITHUB_TOKEN" ]; then \
		echo "Error: GITHUB_TOKEN environment variable is required"; \
		exit 1; \
	fi
	@echo "Logging in to GitHub Container Registry..."
	@echo "$$GITHUB_TOKEN" | docker login ghcr.io -u $$GITHUB_ACTOR --password-stdin
	@echo "Building and pushing multi-arch images to GHCR..."
	docker buildx build --platform linux/amd64,linux/arm64 \
		-t ghcr.io/$$GITHUB_REPOSITORY_OWNER/homewizard-water-exporter:latest \
		-t ghcr.io/$$GITHUB_REPOSITORY_OWNER/homewizard-water-exporter:$$(git describe --tags --always) \
		--push .
	@echo "Successfully pushed to GitHub Container Registry!"

# Run Docker container
docker-run:
	@if [ -z "$$HOMEWIZARD_HOST" ]; then \
		echo "Error: HOMEWIZARD_HOST environment variable is required"; \
		echo "Usage: HOMEWIZARD_HOST=192.168.1.241 make docker-run"; \
		exit 1; \
	fi
	docker run -d --rm \
		--name homewizard-water-exporter \
		-p 9899:9899 \
		-e HOMEWIZARD_HOST=$$HOMEWIZARD_HOST \
		homewizard-water-exporter:latest

# Set GitHub Actions secrets from .env file
gh-secrets:
	@if [ ! -f .env ]; then \
		echo "Error: .env file not found"; \
		echo "Copy .env.example to .env and fill in your values"; \
		exit 1; \
	fi
	@echo "Setting GitHub Actions secrets from .env file..."
	@export $$(cat .env | grep -v '^#' | xargs) && \
		gh secret set DOCKER_USERNAME --body "$$DOCKER_USERNAME" && \
		gh secret set DOCKER_PASSWORD --body "$$DOCKER_PASSWORD" && \
		gh secret set CRATES_IO_TOKEN --body "$$CRATES_IO_TOKEN"
	@echo "GitHub Actions secrets have been set successfully!"