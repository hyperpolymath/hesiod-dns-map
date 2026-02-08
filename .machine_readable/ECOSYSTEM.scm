;; SPDX-License-Identifier: PMPL-1.0-or-later
(ecosystem (metadata (version "0.2.0") (last-updated "2026-02-08"))
  (project (name "hesiod-dns-map") (purpose "Hesiod DNS TXT record to zone file mapper") (role naming-service))
  (flatracoon-integration
    (parent "flatracoon/netstack")
    (layer naming)
    (depended-on-by ("ipv6-site-enforcer"))
    (depends-on ())))
