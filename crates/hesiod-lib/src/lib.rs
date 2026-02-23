// SPDX-License-Identifier: PMPL-1.0-or-later
//! hesiod-lib: Hesiod DNS naming system library.
//!
//! Provides HS-class TXT record management, a lightweight UDP DNS server,
//! and HTTP health/metrics endpoints for FlatRacoon network stack integration.

pub mod config;
pub mod health;
pub mod records;
pub mod server;
pub mod zone;
