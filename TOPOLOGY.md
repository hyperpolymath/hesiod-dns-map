<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# hesiod-dns-map — Project Topology

## System Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              FLATRACOON STACK           │
                        │        (Services, Users, Resources)     │
                        └───────────────────┬─────────────────────┘
                                            │ HS-class Lookups
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │           HESIOD DNS SERVER             │
                        │    (BIND/Knot, HS class records)        │
                        └──────────┬───────────────────┬──────────┘
                                   │                   │
                                   ▼                   ▼
                        ┌───────────────────────┐  ┌────────────────────────────────┐
                        │ HS ZONE FILES         │  │ CONFIGURATION (NICKEL)         │
                        │ - service.hs          │  │ - named.ncl (BIND config)      │
                        │ - passwd.hs / group.hs│  │ - hesiod.ncl (Settings)        │
                        │ - filsys.hs           │  │ - dynamic.ncl (Policies)       │
                        └──────────┬────────────┘  └──────────┬─────────────────────┘
                                   │                          │
                                   └────────────┬─────────────┘
                                                ▼
                        ┌─────────────────────────────────────────┐
                        │           EXTERNAL SYSTEMS              │
                        │  ┌───────────┐  ┌───────────┐  ┌───────┐│
                        │  │ LDAP / AD │  │ Kubernetes│  │ AFS / ││
                        │  │ (Auth)    │  │ (Registry)│  │ NFS   ││
                        │  └───────────┘  └───────────┘  └───────┘│
                        └───────────────────┬─────────────────────┘
                                            │
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │          SERVICE DISCOVERY              │
                        │      (host -t TXT <svc> HS)             │
                        └─────────────────────────────────────────┘

                        ┌─────────────────────────────────────────┐
                        │          REPO INFRASTRUCTURE            │
                        │  Justfile           .machine_readable/  │
                        │  Update Scripts     0-AI-MANIFEST.a2ml  │
                        └─────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
CORE NAMING (HS CLASS)
  Service Registry (.service)       ████████░░  80%    FlatRacoon integration stable
  User/Group Mapping (.passwd)      ██████░░░░  60%    Sync from LDAP in progress
  Filesystem Mapping (.filsys)      ████░░░░░░  40%    AFS/NFS templates active

CONFIGURATION (NICKEL)
  named.ncl (BIND/Knot)             ██████████ 100%    Core config stable
  hesiod.ncl Settings               ██████████ 100%    HS class parameters verified
  Dynamic Update Policies           ██████░░░░  60%    RFC 2136 refining

REPO INFRASTRUCTURE
  Justfile Automation               ██████████ 100%    Zone generation & deploy tasks
  .machine_readable/                ██████████ 100%    STATE.scm tracking
  Zone File Templates               ████████░░  80%    Initial scaffolding verified

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            ███████░░░  ~70%   Hesiod foundation stable
```

## Key Dependencies

```
Nickel Config ───► Zone Generation ───► BIND Server ──────► HS Lookup
     │                 │                   │                 │
     ▼                 ▼                   ▼                 ▼
LDAP Sync ───────► passwd.hs ─────────► Service Disc ───► Client
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
