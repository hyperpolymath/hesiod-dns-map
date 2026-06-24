<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Changelog

All notable changes to `hesiod-dns-map` will be documented in this file.

This file is generated from conventional commits by the
[`changelog-reusable.yml`](https://github.com/hyperpolymath/standards/blob/main/.github/workflows/changelog-reusable.yml)
workflow (`hyperpolymath/standards#206`). Adopt the workflow in this repo's CI to keep this file in sync automatically — see
[`templates/cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml)
for the canonical config.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- feat(crg): add crg-grade and crg-badge justfile recipes
- feat: wire conflow config validation pipeline
- feat: add stapeln.toml layer-based container definition\n\nConverted from existing Containerfile to stapeln format.\nIncludes Chainguard base, security hardening, SBOM generation.\n\nCo-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
- feat: deploy UX Manifesto infrastructure
- feat: add CLADE.a2ml — clade taxonomy declaration
- feat: add bot directives, contractiles, and ecosystem cross-refs
- feat: implement Hesiod DNS naming system with Rust workspace\n\nRust workspace with two crates:\n- hesiod-lib: HS-class TXT record types (passwd, group, service, filsys),\n  zone management, UDP DNS server (hickory-proto), Axum health/metrics\n- hesinfo: CLI with lookup, serve, generate, validate subcommands\n\nAlso includes Nickel config schemas, K8s manifests, FlatRacoon module\nmanifest, Containerfile, example zone file, and real justfile recipes.\n\n16 tests passing, clippy clean.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
- feat: add AI Gatekeeper Protocol manifest
- feat(ci): enable Hypatia scanning

### Fixed

- fix(ci): bump a2ml/k9-validate-action pins to canonical (#44)
- fix(ci): sync hypatia-scan.yml to canonical (#43)
- fix(ci): build Hypatia escript from repo root (estate dogfood drift)
- fix: set correct Groove capability type (was: custom)
- fix(scorecard): enforce granular permissions and add fuzzing placeholder
- fix(ci): Resolve workflow-linter self-matching and metadata issues
- fix(scorecard): enforce granular permissions and add fuzzing placeholder
- fix(ci): Resolve workflow-linter self-matching and metadata issues
- fix: correct email jonathan.jewell → j.d.a.jewell
- fix: SPDX headers (AGPL→PMPL), email, author name

### Changed

- refactor: migrate 6SCM → 6A2 (.scm → .a2ml format)

### Documentation

- docs: record tech-debt audit findings (2026-05-26) (#49)
- docs(governance): CRG v2.0 STRICT audit — C (declared) -> D (honest)
- docs: add EXPLAINME.adoc — prove-it file backing README claims
- docs: add CONTRIBUTING.md
- docs: add checkpoint files for state tracking

### CI

- ci(rust): convert rust-ci.yml to thin wrapper (standards#174) (#48)
- ci: redistribute concurrency-cancel guard to read-only check workflows (#46)
- ci: bump actions/upload-artifact SHA to current v4 (#42)
- ci: SHA-pin hyperpolymath validate-actions in dogfood-gate
- ci: wire hypatia-scan.yml to query own Dependabot alerts

## Pre-history

Prior commits to this file's introduction are recorded in git history but not formally classified into Keep-a-Changelog sections. To backfill, run `git cliff -o CHANGELOG.md` locally using the canonical [`cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml) — this is one-shot mechanical work.

---

<!-- This file was seeded by the 2026-05-26 estate tech-debt audit follow-up (Row-2 Phase 3); see [`hyperpolymath/standards/docs/audits/2026-05-26-estate-documentation-debt.md`](https://github.com/hyperpolymath/standards/blob/main/docs/audits/2026-05-26-estate-documentation-debt.md). -->
