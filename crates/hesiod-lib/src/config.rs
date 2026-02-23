// SPDX-License-Identifier: PMPL-1.0-or-later
//! Configuration loading from JSON (produced by `nickel export`).

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Top-level Hesiod configuration matching the Nickel schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HesiodConfig {
    pub domain: String,
    pub lhs: String,
    pub rhs: String,
    #[serde(default = "default_ttl")]
    pub ttl: u32,
    #[serde(default = "default_dns_port")]
    pub dns_port: u16,
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    #[serde(default)]
    pub services: Vec<ServiceEntry>,
    #[serde(default)]
    pub users: Vec<UserEntry>,
    #[serde(default)]
    pub groups: Vec<GroupEntry>,
}

fn default_ttl() -> u32 {
    300
}
fn default_dns_port() -> u16 {
    53
}
fn default_http_port() -> u16 {
    8080
}

/// Service entry from config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEntry {
    pub name: String,
    pub host: String,
    pub port: u16,
    #[serde(default = "default_protocol")]
    pub protocol: String,
}

fn default_protocol() -> String {
    "tcp".into()
}

/// User entry from config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    #[serde(default)]
    pub gecos: String,
    pub home: String,
    #[serde(default = "default_shell")]
    pub shell: String,
}

fn default_shell() -> String {
    "/bin/bash".into()
}

/// Group entry from config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEntry {
    pub name: String,
    pub gid: u32,
    #[serde(default)]
    pub members: Vec<String>,
}

impl HesiodConfig {
    /// Load configuration from a JSON file (output of `nickel export`).
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("reading config from {}", path.display()))?;
        Self::from_json(&content)
    }

    /// Parse configuration from a JSON string.
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).context("parsing Hesiod config JSON")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_config() {
        let json = r#"{
            "domain": "example.internal",
            "lhs": ".ns",
            "rhs": ".example.internal"
        }"#;
        let config = HesiodConfig::from_json(json).unwrap();
        assert_eq!(config.domain, "example.internal");
        assert_eq!(config.ttl, 300);
        assert_eq!(config.dns_port, 53);
        assert!(config.services.is_empty());
    }

    #[test]
    fn parse_full_config() {
        let json = r#"{
            "domain": "flatracoon.internal",
            "lhs": ".ns",
            "rhs": ".flatracoon.internal",
            "ttl": 600,
            "dns_port": 5353,
            "http_port": 9090,
            "services": [
                {"name": "web", "host": "web.svc", "port": 443, "protocol": "tcp"}
            ],
            "users": [
                {"username": "admin", "uid": 1000, "gid": 1000, "gecos": "Admin", "home": "/home/admin", "shell": "/bin/zsh"}
            ],
            "groups": [
                {"name": "ops", "gid": 1001, "members": ["admin"]}
            ]
        }"#;
        let config = HesiodConfig::from_json(json).unwrap();
        assert_eq!(config.ttl, 600);
        assert_eq!(config.services.len(), 1);
        assert_eq!(config.users.len(), 1);
        assert_eq!(config.groups.len(), 1);
        assert_eq!(config.users[0].shell, "/bin/zsh");
    }
}
