# SPDX-License-Identifier: PMPL-1.0-or-later
# Multi-stage build for hesiod-dns-map

FROM rust:1.85-slim AS builder
WORKDIR /build
COPY Cargo.toml Cargo.toml
COPY crates/ crates/
RUN cargo build --release --bin hesinfo

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /build/target/release/hesinfo /usr/local/bin/hesinfo
EXPOSE 53/udp 53/tcp 8080/tcp
USER nonroot:nonroot
ENTRYPOINT ["hesinfo"]
CMD ["serve", "--config", "/etc/hesiod/config.json"]
