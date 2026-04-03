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

# [AUTO-GENERATED] Multi-arch / RISC-V target
build-riscv:
	@echo "Building for RISC-V..."
	cross build --target riscv64gc-unknown-linux-gnu

# Run panic-attacker pre-commit scan
assail:
    @command -v panic-attack >/dev/null 2>&1 && panic-attack assail . || echo "panic-attack not found — install from https://github.com/hyperpolymath/panic-attacker"

# Self-diagnostic — checks dependencies, permissions, paths
doctor:
    @echo "Running diagnostics for hesiod-dns-map..."
    @echo "Checking required tools..."
    @command -v just >/dev/null 2>&1 && echo "  [OK] just" || echo "  [FAIL] just not found"
    @command -v git >/dev/null 2>&1 && echo "  [OK] git" || echo "  [FAIL] git not found"
    @echo "Checking for hardcoded paths..."
    @grep -rn '$HOME\|$ECLIPSE_DIR' --include='*.rs' --include='*.ex' --include='*.res' --include='*.gleam' --include='*.sh' . 2>/dev/null | head -5 || echo "  [OK] No hardcoded paths"
    @echo "Diagnostics complete."

# Auto-repair common issues
heal:
    @echo "Attempting auto-repair for hesiod-dns-map..."
    @echo "Fixing permissions..."
    @find . -name "*.sh" -exec chmod +x {} \; 2>/dev/null || true
    @echo "Cleaning stale caches..."
    @rm -rf .cache/stale 2>/dev/null || true
    @echo "Repair complete."

# Guided tour of key features
tour:
    @echo "=== hesiod-dns-map Tour ==="
    @echo ""
    @echo "1. Project structure:"
    @ls -la
    @echo ""
    @echo "2. Available commands: just --list"
    @echo ""
    @echo "3. Read README.adoc for full overview"
    @echo "4. Read EXPLAINME.adoc for architecture decisions"
    @echo "5. Run 'just doctor' to check your setup"
    @echo ""
    @echo "Tour complete! Try 'just --list' to see all available commands."

# Open feedback channel with diagnostic context
help-me:
    @echo "=== hesiod-dns-map Help ==="
    @echo "Platform: $(uname -s) $(uname -m)"
    @echo "Shell: $SHELL"
    @echo ""
    @echo "To report an issue:"
    @echo "  https://github.com/hyperpolymath/hesiod-dns-map/issues/new"
    @echo ""
    @echo "Include the output of 'just doctor' in your report."
