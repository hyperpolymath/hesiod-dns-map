# SPDX-License-Identifier: PMPL-1.0-or-later
# Justfile for hesiod-dns-map

default:
    @just --list

# Build all crates in release mode
build:
    cargo build --release

# Run all tests
test:
    cargo test

# Start the Hesiod DNS server (dev mode, port 5353)
serve port="5353" http_port="8080":
    cargo run --bin hesinfo -- serve --dns-port {{port}} --http-port {{http_port}} --config configs/hesiod/flatracoon.json

# Look up a Hesiod record
lookup key map server="localhost" port="5353":
    cargo run --bin hesinfo -- lookup {{key}} {{map}} --server {{server}} --port {{port}}

# Generate BIND-format zone files from Nickel config
generate-zones:
    cargo run --bin hesinfo -- generate --config configs/hesiod/flatracoon.json --output zones/generated.hs

# Export Nickel config to JSON
config-export:
    nickel export configs/hesiod/flatracoon.ncl > configs/hesiod/flatracoon.json

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting without modifying
fmt-check:
    cargo fmt -- --check

# Run all checks (lint + test + fmt)
check: fmt-check lint test

# Clean build artifacts
clean:
    cargo clean

# Build container image
container-build tag="latest":
    podman build -f Containerfile -t ghcr.io/hyperpolymath/hesiod-dns-map:{{tag}} .

# Deploy to Kubernetes
deploy:
    kubectl apply -f manifests/

# Validate zone file syntax
validate file:
    cargo run --bin hesinfo -- validate {{file}}
