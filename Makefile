# askr - Interactive CLI input tool
# Makefile for building, testing, and releasing

.PHONY: help build test clean lint fmt check install uninstall
.PHONY: release-dry release tag docs examples
.PHONY: completion completions pre-commit setup
.DEFAULT_GOAL := help

# Build configuration
CARGO := cargo
TARGET_DIR := target
BINARY_NAME := askr
VERSION := $(shell grep '^version =' Cargo.toml | head -1 | cut -d '"' -f 2)

# Colors for output
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
BOLD := \033[1m
RESET := \033[0m

help: ## Show this help message
	@echo "$(BOLD)askr v$(VERSION) - Interactive CLI input tool$(RESET)"
	@echo ""
	@echo "$(BOLD)Available targets:$(RESET)"
	@awk 'BEGIN {FS = ":.*##"; printf ""} /^[a-zA-Z_-]+:.*?##/ { printf "  $(GREEN)%-15s$(RESET) %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

# Development targets
build: ## Build the project in debug mode
	@echo "$(BLUE)Building $(BINARY_NAME)...$(RESET)"
	$(CARGO) build

build-release: ## Build the project in release mode
	@echo "$(BLUE)Building $(BINARY_NAME) (release)...$(RESET)"
	$(CARGO) build --release

test: ## Run all tests
	@echo "$(BLUE)Running tests...$(RESET)"
	$(CARGO) test

test-verbose: ## Run tests with verbose output
	@echo "$(BLUE)Running tests (verbose)...$(RESET)"
	$(CARGO) test -- --nocapture

check: ## Check code without building
	@echo "$(BLUE)Checking code...$(RESET)"
	$(CARGO) check

# Code quality targets
lint: ## Run clippy lints
	@echo "$(BLUE)Running clippy...$(RESET)"
	$(CARGO) clippy --all-targets --all-features -- -D unused-imports -D clippy::collapsible_else_if -D clippy::int_plus_one

lint-fix: ## Run clippy with automatic fixes
	@echo "$(BLUE)Running clippy with fixes...$(RESET)"
	$(CARGO) clippy --fix --allow-dirty --allow-staged

fmt: ## Format code with rustfmt
	@echo "$(BLUE)Formatting code...$(RESET)"
	$(CARGO) fmt

fmt-check: ## Check code formatting
	@echo "$(BLUE)Checking code formatting...$(RESET)"
	$(CARGO) fmt --check

# Installation targets
install: build-release ## Install the binary locally
	@echo "$(BLUE)Installing $(BINARY_NAME)...$(RESET)"
	$(CARGO) install --path .

uninstall: ## Uninstall the binary
	@echo "$(BLUE)Uninstalling $(BINARY_NAME)...$(RESET)"
	$(CARGO) uninstall $(BINARY_NAME)

# Release targets
release-dry: ## Dry run of crates.io release
	@echo "$(BLUE)Dry run release to crates.io...$(RESET)"
	$(CARGO) publish --dry-run

release: ## DEPRECATED: Use 'make tag' instead for CI-driven releases
	@echo "$(RED)❌ Direct publishing is deprecated!$(RESET)"
	@echo "$(YELLOW)Use the new CI-driven release process:$(RESET)"
	@echo "  1. $(BLUE)make release-dry$(RESET)     # Verify package"
	@echo "  2. $(BLUE)git add Cargo.toml Cargo.lock$(RESET)"
	@echo "  3. $(BLUE)git commit -m 'Bump version to v$(VERSION)'$(RESET)"
	@echo "  4. $(BLUE)make tag$(RESET)            # Creates tag and triggers CI release"
	@echo ""
	@echo "$(GREEN)✅ This ensures CI validation before publishing to crates.io$(RESET)"
	@exit 1

tag: ## Create and push a git tag for current version
	@echo "$(BLUE)Creating tag v$(VERSION)...$(RESET)"
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	git push origin v$(VERSION)
	@echo "$(GREEN)Tag v$(VERSION) created and pushed!$(RESET)"

# Documentation targets
docs: ## Generate and open documentation
	@echo "$(BLUE)Generating documentation...$(RESET)"
	$(CARGO) doc --open

docs-build: ## Build documentation without opening
	@echo "$(BLUE)Building documentation...$(RESET)"
	$(CARGO) doc

# Shell completion targets
completion: completions ## Alias for completions

completions: build ## Generate shell completions
	@echo "$(BLUE)Generating shell completions...$(RESET)"
	@mkdir -p completions
	./$(TARGET_DIR)/debug/$(BINARY_NAME) completion bash > completions/$(BINARY_NAME).bash
	./$(TARGET_DIR)/debug/$(BINARY_NAME) completion zsh > completions/$(BINARY_NAME).zsh
	./$(TARGET_DIR)/debug/$(BINARY_NAME) completion fish > completions/$(BINARY_NAME).fish
	./$(TARGET_DIR)/debug/$(BINARY_NAME) completion power-shell > completions/$(BINARY_NAME).ps1
	@echo "$(GREEN)Completions generated in completions/$(RESET)"

# Example targets
examples: build ## Run example scripts
	@echo "$(BLUE)Running example scripts...$(RESET)"
	@if [ -d examples ]; then \
		for script in examples/*.sh; do \
			echo "$(YELLOW)Running $$script...$(RESET)"; \
			bash "$$script"; \
		done; \
	else \
		echo "$(RED)No examples directory found$(RESET)"; \
	fi

# Development setup targets
setup: ## Set up development environment
	@echo "$(BLUE)Setting up development environment...$(RESET)"
	@command -v pre-commit >/dev/null 2>&1 || { \
		echo "$(YELLOW)Installing pre-commit...$(RESET)"; \
		pip install pre-commit; \
	}
	@echo "$(BLUE)Installing pre-commit hooks...$(RESET)"
	pre-commit install
	@echo "$(GREEN)Development environment ready!$(RESET)"

pre-commit: ## Run pre-commit hooks manually
	@echo "$(BLUE)Running pre-commit hooks...$(RESET)"
	pre-commit run --all-files

# CI targets (for local verification)
ci-check: fmt-check lint test ## Run all CI checks locally
	@echo "$(GREEN)All CI checks passed!$(RESET)"

ci-build: ## Build for CI (all targets)
	@echo "$(BLUE)Building all targets for CI...$(RESET)"
	$(CARGO) build --all-targets
	$(CARGO) build --all-targets --release

# Utility targets
clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(RESET)"
	$(CARGO) clean
	rm -rf completions/

update: ## Update dependencies
	@echo "$(BLUE)Updating dependencies...$(RESET)"
	$(CARGO) update

audit: ## Run security audit
	@echo "$(BLUE)Running security audit...$(RESET)"
	$(CARGO) audit

# Version management
version: ## Show current version
	@echo "$(BOLD)Current version: $(GREEN)$(VERSION)$(RESET)"

bump-patch: ## Bump patch version (x.y.Z)
	@echo "$(BLUE)Bumping patch version...$(RESET)"
	@current=$$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f 2); \
	new=$$(echo $$current | awk -F. '{print $$1"."$$2"."$$3+1}'); \
	sed -i.bak "s/^version = \"$$current\"/version = \"$$new\"/" Cargo.toml; \
	rm Cargo.toml.bak; \
	echo "$(GREEN)Version bumped from $$current to $$new$(RESET)"

bump-minor: ## Bump minor version (x.Y.z)
	@echo "$(BLUE)Bumping minor version...$(RESET)"
	@current=$$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f 2); \
	new=$$(echo $$current | awk -F. '{print $$1"."$$2+1".0"}'); \
	sed -i.bak "s/^version = \"$$current\"/version = \"$$new\"/" Cargo.toml; \
	rm Cargo.toml.bak; \
	echo "$(GREEN)Version bumped from $$current to $$new$(RESET)"

bump-major: ## Bump major version (X.y.z)
	@echo "$(BLUE)Bumping major version...$(RESET)"
	@current=$$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f 2); \
	new=$$(echo $$current | awk -F. '{print $$1+1".0.0"}'); \
	sed -i.bak "s/^version = \"$$current\"/version = \"$$new\"/" Cargo.toml; \
	rm Cargo.toml.bak; \
	echo "$(GREEN)Version bumped from $$current to $$new$(RESET)"

# Release workflow
release-workflow: ci-check release-dry ## Run complete release checks
	@echo "$(GREEN)Release workflow complete. Ready to tag and release!$(RESET)"
	@echo "$(YELLOW)Next steps:$(RESET)"
	@echo "  1. $(BLUE)git add Cargo.toml Cargo.lock$(RESET)"
	@echo "  2. $(BLUE)git commit -m 'Bump version to v$(VERSION)'$(RESET)"
	@echo "  3. $(BLUE)make tag$(RESET)  # This triggers CI-driven release"

prepare-release: ## Prepare a new release (bump version, run checks)
	@echo "$(BLUE)Preparing release for $(BINARY_NAME)...$(RESET)"
	@echo "$(YELLOW)Current version: $(VERSION)$(RESET)"
	@echo ""
	@echo "$(BLUE)1. Update version in Cargo.toml manually$(RESET)"
	@echo "$(BLUE)2. Run: make release-workflow$(RESET)"
	@echo "$(BLUE)3. Run: make tag$(RESET)"
	@echo ""
	@echo "$(GREEN)✅ CI will handle testing and publishing automatically$(RESET)"

# Quick targets for common tasks
all: clean build test lint ## Clean, build, test, and lint
dev: build test ## Quick development build and test
ready: ci-check build-release ## Verify project is ready for release