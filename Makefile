# Makefile for System Monitor + Webhook Server

CONFIG ?= ./src/config.toml

# Build both binaries
build:
	echo "Building system monitor and webhook server..."
	cargo build --release
	echo "For webook, don't forget to run 'make tailwindcss' to generate the CSS, and deploy the static files."

# Run the system monitor with optional config path
run:
	cargo run --bin system_monitor -- --config $(CONFIG)

# Run the webhook test server
webhook:
	cargo run --bin webhook_server

# Clean build artifacts
clean:
	cargo clean

# Build and run the system monitor
start: build run

test:
	cargo test

docs:
	cargo doc --no-deps --open
	rm -rf docs/*
	cp -r target/doc/* docs/
	# Create an index.html file to redirect to system_monitor
	echo '<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta http-equiv="refresh" content="0; url=system_monitor/"><title>Redirecting...</title></head><body><p>If you are not redirected automatically, follow this <a href="system_monitor/">link</a>.</p></body></html>' > docs/index.html


tailwindcss:
	BROWSERSLIST_IGNORE_OLD_DATA=1 npx tailwindcss -i ./frontend/input.css -o ./static/main.css

# Print help
help:
	@echo "Makefile commands:"
	@echo "  make build           # Build all binaries in release mode"
	@echo "  make run CONFIG=path/to/config.toml"
	@echo "                       # Run system monitor (default config: ./src/config.toml)"
	@echo "  make webhook         # Run sample webhook test server"
	@echo "  make clean           # Clean build artifacts"
	@echo "  make start           # Build and run system monitor"
