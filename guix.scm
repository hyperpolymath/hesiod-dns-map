; SPDX-License-Identifier: PMPL-1.0-or-later
;; guix.scm — GNU Guix package definition for hesiod-dns-map
;; Usage: guix shell -f guix.scm

(use-modules (guix packages)
             (guix build-system gnu)
             (guix licenses))

(package
  (name "hesiod-dns-map")
  (version "0.1.0")
  (source #f)
  (build-system gnu-build-system)
  (synopsis "hesiod-dns-map")
  (description "hesiod-dns-map — part of the hyperpolymath ecosystem.")
  (home-page "https://github.com/hyperpolymath/hesiod-dns-map")
  (license ((@@ (guix licenses) license) "PMPL-1.0-or-later"
             "https://github.com/hyperpolymath/palimpsest-license")))
