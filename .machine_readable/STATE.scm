;; SPDX-License-Identifier: PMPL-1.0-or-later
;; STATE.scm - Project state for hesiod-dns-map
;; Media-Type: application/vnd.state+scm

(state
  (metadata
    (version "0.1.0")
    (schema-version "1.0")
    (created "2026-01-03")
    (updated "2026-02-08")
    (project "hesiod-dns-map")
    (repo "github.com/hyperpolymath/hesiod-dns-map"))

  (project-context
    (name "hesiod-dns-map")
    (tagline "Hesiod DNS-based naming and service discovery using HS-class TXT records")
    (tech-stack ("rust" "hickory-proto" "tokio" "axum" "nickel" "kubernetes")))

  (current-position
    (phase "mvp")
    (overall-completion 85)
    (components
      ("hesiod-lib" "core library with records, zone, server, health, config" 90)
      ("hesinfo" "CLI tool with lookup, serve, generate, validate" 90)
      ("nickel-config" "type-safe configuration schemas" 100)
      ("k8s-manifests" "deployment, service, configmap" 100)
      ("flatracoon-integration" "module manifest for orchestrator discovery" 100))
    (working-features
      ("passwd-record-roundtrip" "PasswdRecord TXT serialization/deserialization")
      ("group-record-roundtrip" "GroupRecord TXT serialization/deserialization")
      ("service-record-roundtrip" "ServiceRecord TXT serialization/deserialization")
      ("filsys-record-roundtrip" "FilsysRecord TXT serialization/deserialization")
      ("zone-management" "HesiodZone with HashMap-based record storage")
      ("bind-zone-export" "BIND-format zone file generation")
      ("udp-dns-server" "hickory-proto based HS-class DNS server")
      ("http-health" "Axum health/metrics endpoints on port 8080")
      ("json-config" "Configuration from nickel export JSON")
      ("cli-serve" "hesinfo serve - start DNS + HTTP server")
      ("cli-lookup" "hesinfo lookup - DNS query client")
      ("cli-generate" "hesinfo generate - zone file generator")
      ("cli-validate" "hesinfo validate - zone file validator")))

  (route-to-mvp
    (milestones
      ("m1-core" "Core library implementation" 90
        ("record types" "zone management" "DNS server" "health endpoints" "config loading"))
      ("m2-cli" "CLI tool" 90
        ("lookup" "serve" "generate" "validate"))
      ("m3-config" "Nickel configuration" 100
        ("schema contracts" "flatracoon config"))
      ("m4-k8s" "Kubernetes deployment" 100
        ("deployment" "service" "configmap"))
      ("m5-integration" "FlatRacoon integration" 100
        ("module manifest" "orchestrator discovery"))))

  (blockers-and-issues
    (critical)
    (high)
    (medium
      ("zone-reload" "POST /dns/reload not yet implemented"))
    (low
      ("tcp-dns" "TCP DNS not yet supported, UDP only")))

  (critical-next-actions
    (immediate
      ("verify-build" "cargo build --release must compile")
      ("verify-tests" "cargo test must pass"))
    (this-week
      ("integration-test" "test with dig against running server"))
    (this-month
      ("zone-reload" "implement hot zone reload")))

  (session-history
    ("2026-02-08" "initial implementation: rust workspace, hesiod-lib, hesinfo CLI, nickel config, k8s manifests, containerfile, flatracoon module manifest")))
