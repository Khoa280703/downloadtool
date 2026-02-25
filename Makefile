# Video Downloader - Build System
# ================================

# Default target
.PHONY: all
all: build

# Directories
EXTRACTORS_DIR := extractors
EXTRACTORS_DIST := $(EXTRACTORS_DIR)/dist
CRATES_DIR := crates

# Build commands
.PHONY: build
build: bundle-extractors
	cargo build --release

.PHONY: build-dev
dev-build:
	cargo build

# Bundle TypeScript extractors using esbuild
.PHONY: bundle-extractors
bundle-extractors: $(EXTRACTORS_DIST)
	npx esbuild $(EXTRACTORS_DIR)/youtube.ts \
		--bundle \
		--outfile=$(EXTRACTORS_DIST)/youtube.js \
		--format=esm \
		--platform=neutral \
		--target=es2020
	npx esbuild $(EXTRACTORS_DIR)/types.ts \
		--bundle \
		--outfile=$(EXTRACTORS_DIST)/types.js \
		--format=esm \
		--platform=neutral \
		--target=es2020
	@echo "Bundled extractors to $(EXTRACTORS_DIST)/"

# Create combined bundle for Rust embedding
.PHONY: bundle-combined
bundle-combined: bundle-extractors
	@echo "Creating combined bundle..."
	@cat $(EXTRACTORS_DIST)/types.js > $(EXTRACTORS_DIST)/bundle.js
	@echo "" >> $(EXTRACTORS_DIST)/bundle.js
	@cat $(EXTRACTORS_DIST)/youtube.js >> $(EXTRACTORS_DIST)/bundle.js
	@echo "" >> $(EXTRACTORS_DIST)/bundle.js
	@echo "const extractors = { youtube };" >> $(EXTRACTORS_DIST)/bundle.js
	@echo "Combined bundle created at $(EXTRACTORS_DIST)/bundle.js"

# Create dist directory
$(EXTRACTORS_DIR)/dist:
	mkdir -p $(EXTRACTORS_DIR)/dist

# Watch mode for development
.PHONY: watch-extractors
watch-extractors: $(EXTRACTORS_DIST)
	npx esbuild $(EXTRACTORS_DIR)/youtube.ts \
		--bundle \
		--outfile=$(EXTRACTORS_DIST)/youtube.js \
		--format=esm \
		--platform=neutral \
		--target=es2020 \
		--watch &
	@echo "Watching extractors for changes..."

# Test commands
.PHONY: test
test: bundle-extractors
	cargo test --workspace

.PHONY: test-extractor
test-extractor: bundle-extractors
	cargo test -p extractor

# Lint and format
.PHONY: lint
lint:
	cargo clippy --workspace -- -D warnings
	cd $(EXTRACTORS_DIR) && npx tsc --noEmit 2>/dev/null || true

.PHONY: fmt
fmt:
	cargo fmt --all
	cd $(EXTRACTORS_DIR) && npx prettier --write "*.ts" 2>/dev/null || true

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
	rm -rf $(EXTRACTORS_DIST)

# Install dependencies
.PHONY: install-deps
install-deps:
	cargo fetch
	cd $(EXTRACTORS_DIR) && npm install esbuild typescript @types/node 2>/dev/null || npm install

# Development server
.PHONY: dev
dev: bundle-extractors
	cargo run --bin server

# Docker commands
.PHONY: docker-build
docker-build:
	docker build -t video-downloader:latest .

.PHONY: docker-run
docker-run:
	docker run -p 3068:3068 video-downloader:latest

# Help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all              - Build everything (default)"
	@echo "  build            - Build release binary"
	@echo "  build-dev        - Build development binary"
	@echo "  bundle-extractors - Bundle TypeScript extractors with esbuild"
	@echo "  bundle-combined  - Create combined bundle for embedding"
	@echo "  watch-extractors - Watch extractors and rebuild on changes"
	@echo "  test             - Run all tests"
	@echo "  test-extractor   - Run extractor crate tests"
	@echo "  lint             - Run linter"
	@echo "  fmt              - Format code"
	@echo "  clean            - Clean build artifacts"
	@echo "  install-deps     - Install dependencies"
	@echo "  dev              - Run development server"
	@echo "  docker-build     - Build Docker image"
	@echo "  docker-run       - Run Docker container"
