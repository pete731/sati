.PHONY: demo test build deploy clean

# Run the SATI demo
demo:
	@node scripts/demo.js

# Build all programs
build:
	anchor build

# Run tests
test:
	anchor test

# Deploy to devnet
deploy:
	anchor deploy --provider.cluster devnet

# Clean build artifacts
clean:
	rm -rf target/

# Install dependencies
install:
	pnpm install
	cd sdk && pnpm install

# Quick start for demos
quickstart: install build demo

help:
	@echo "SATI - Solana Agent Trust Infrastructure"
	@echo ""
	@echo "Available commands:"
	@echo "  make demo       - Run the SATI demo"
	@echo "  make build      - Build Anchor programs"
	@echo "  make test       - Run tests"
	@echo "  make deploy     - Deploy to devnet"
	@echo "  make quickstart - Install, build, and run demo"
	@echo "  make clean      - Clean build artifacts"