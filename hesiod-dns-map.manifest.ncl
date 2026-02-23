# SPDX-License-Identifier: PMPL-1.0-or-later
# hesiod-dns-map module manifest for FlatRacoon orchestrator

{
  name = "hesiod-dns-map",
  version = "0.1.0",
  layer = 'naming,
  description = "Hesiod DNS-based naming and service discovery using HS-class TXT records",
  repo = "github.com/hyperpolymath/hesiod-dns-map",
  requires = ["kubernetes"],
  provides = ["hesiod-dns", "service-discovery", "user-lookup"],
  config_schema = "hesiod/schema.ncl",
  health_endpoint = "/dns/health",
  metrics_endpoint = "/dns/metrics",
  deployment_mode = "kubectl",
  namespace = "flatracoon",
}
