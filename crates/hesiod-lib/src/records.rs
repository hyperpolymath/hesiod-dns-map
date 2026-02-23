// SPDX-License-Identifier: PMPL-1.0-or-later
//! Hesiod record types: Passwd, Group, Service, Filsys
//! Each record supports round-trip TXT serialization.

use std::fmt;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

/// Map types corresponding to Hesiod naming conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MapType {
    Passwd,
    Group,
    Service,
    Filsys,
}

impl MapType {
    /// DNS label used in zone names (e.g. `admin.passwd.ns`).
    pub fn label(&self) -> &'static str {
        match self {
            MapType::Passwd => "passwd",
            MapType::Group => "group",
            MapType::Service => "service",
            MapType::Filsys => "filsys",
        }
    }
}

impl fmt::Display for MapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

impl std::str::FromStr for MapType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "passwd" => Ok(MapType::Passwd),
            "group" => Ok(MapType::Group),
            "service" => Ok(MapType::Service),
            "filsys" => Ok(MapType::Filsys),
            other => bail!("unknown map type: {other}"),
        }
    }
}

// ---------------------------------------------------------------------------
// PasswdRecord
// ---------------------------------------------------------------------------

/// Unix passwd entry: `user:*:uid:gid:gecos:home:shell`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswdRecord {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub gecos: String,
    pub home: String,
    pub shell: String,
}

impl PasswdRecord {
    pub fn to_txt(&self) -> String {
        format!(
            "{}:*:{}:{}:{}:{}:{}",
            self.username, self.uid, self.gid, self.gecos, self.home, self.shell,
        )
    }

    pub fn from_txt(txt: &str) -> Result<Self> {
        let parts: Vec<&str> = txt.splitn(7, ':').collect();
        if parts.len() != 7 {
            bail!("passwd record requires 7 colon-separated fields, got {}", parts.len());
        }
        Ok(Self {
            username: parts[0].to_string(),
            // parts[1] is the password placeholder (always "*")
            uid: parts[2].parse().context("invalid uid")?,
            gid: parts[3].parse().context("invalid gid")?,
            gecos: parts[4].to_string(),
            home: parts[5].to_string(),
            shell: parts[6].to_string(),
        })
    }
}

impl fmt::Display for PasswdRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_txt())
    }
}

// ---------------------------------------------------------------------------
// GroupRecord
// ---------------------------------------------------------------------------

/// Unix group entry: `group:*:gid:member1,member2`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupRecord {
    pub name: String,
    pub gid: u32,
    pub members: Vec<String>,
}

impl GroupRecord {
    pub fn to_txt(&self) -> String {
        format!("{}:*:{}:{}", self.name, self.gid, self.members.join(","))
    }

    pub fn from_txt(txt: &str) -> Result<Self> {
        let parts: Vec<&str> = txt.splitn(4, ':').collect();
        if parts.len() != 4 {
            bail!("group record requires 4 colon-separated fields, got {}", parts.len());
        }
        let members = if parts[3].is_empty() {
            Vec::new()
        } else {
            parts[3].split(',').map(|s| s.to_string()).collect()
        };
        Ok(Self {
            name: parts[0].to_string(),
            // parts[1] is the password placeholder (always "*")
            gid: parts[2].parse().context("invalid gid")?,
            members,
        })
    }
}

impl fmt::Display for GroupRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_txt())
    }
}

// ---------------------------------------------------------------------------
// ServiceRecord
// ---------------------------------------------------------------------------

/// Service location: `host:port:protocol`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRecord {
    pub host: String,
    pub port: u16,
    pub protocol: String,
}

impl ServiceRecord {
    pub fn to_txt(&self) -> String {
        format!("{}:{}:{}", self.host, self.port, self.protocol)
    }

    pub fn from_txt(txt: &str) -> Result<Self> {
        let parts: Vec<&str> = txt.splitn(3, ':').collect();
        if parts.len() != 3 {
            bail!("service record requires 3 colon-separated fields, got {}", parts.len());
        }
        Ok(Self {
            host: parts[0].to_string(),
            port: parts[1].parse().context("invalid port")?,
            protocol: parts[2].to_string(),
        })
    }
}

impl fmt::Display for ServiceRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_txt())
    }
}

// ---------------------------------------------------------------------------
// FilsysRecord
// ---------------------------------------------------------------------------

/// Filesystem mount: `type path server:export mode`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilsysRecord {
    pub fs_type: String,
    pub mount_path: String,
    pub source: String,
    pub mode: String,
}

impl FilsysRecord {
    pub fn to_txt(&self) -> String {
        format!("{} {} {} {}", self.fs_type, self.mount_path, self.source, self.mode)
    }

    pub fn from_txt(txt: &str) -> Result<Self> {
        let parts: Vec<&str> = txt.splitn(4, ' ').collect();
        if parts.len() != 4 {
            bail!("filsys record requires 4 space-separated fields, got {}", parts.len());
        }
        Ok(Self {
            fs_type: parts[0].to_string(),
            mount_path: parts[1].to_string(),
            source: parts[2].to_string(),
            mode: parts[3].to_string(),
        })
    }
}

impl fmt::Display for FilsysRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_txt())
    }
}

// ---------------------------------------------------------------------------
// HesiodRecord enum
// ---------------------------------------------------------------------------

/// Unified enum wrapping all Hesiod record types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum HesiodRecord {
    Passwd(PasswdRecord),
    Group(GroupRecord),
    Service(ServiceRecord),
    Filsys(FilsysRecord),
}

impl HesiodRecord {
    pub fn map_type(&self) -> MapType {
        match self {
            HesiodRecord::Passwd(_) => MapType::Passwd,
            HesiodRecord::Group(_) => MapType::Group,
            HesiodRecord::Service(_) => MapType::Service,
            HesiodRecord::Filsys(_) => MapType::Filsys,
        }
    }

    pub fn to_txt(&self) -> String {
        match self {
            HesiodRecord::Passwd(r) => r.to_txt(),
            HesiodRecord::Group(r) => r.to_txt(),
            HesiodRecord::Service(r) => r.to_txt(),
            HesiodRecord::Filsys(r) => r.to_txt(),
        }
    }

    pub fn from_txt(map_type: MapType, txt: &str) -> Result<Self> {
        match map_type {
            MapType::Passwd => Ok(HesiodRecord::Passwd(PasswdRecord::from_txt(txt)?)),
            MapType::Group => Ok(HesiodRecord::Group(GroupRecord::from_txt(txt)?)),
            MapType::Service => Ok(HesiodRecord::Service(ServiceRecord::from_txt(txt)?)),
            MapType::Filsys => Ok(HesiodRecord::Filsys(FilsysRecord::from_txt(txt)?)),
        }
    }

    /// DNS name used for this record (e.g. `admin` for a passwd lookup of user admin).
    pub fn key(&self) -> &str {
        match self {
            HesiodRecord::Passwd(r) => &r.username,
            HesiodRecord::Group(r) => &r.name,
            HesiodRecord::Service(r) => &r.host,
            HesiodRecord::Filsys(r) => &r.mount_path,
        }
    }
}

impl fmt::Display for HesiodRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_txt())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passwd_round_trip() {
        let record = PasswdRecord {
            username: "admin".into(),
            uid: 1000,
            gid: 1000,
            gecos: "FlatRacoon Admin".into(),
            home: "/home/admin".into(),
            shell: "/bin/bash".into(),
        };
        let txt = record.to_txt();
        assert_eq!(txt, "admin:*:1000:1000:FlatRacoon Admin:/home/admin:/bin/bash");
        let parsed = PasswdRecord::from_txt(&txt).unwrap();
        assert_eq!(record, parsed);
    }

    #[test]
    fn group_round_trip() {
        let record = GroupRecord {
            name: "operators".into(),
            gid: 1001,
            members: vec!["admin".into(), "operator".into()],
        };
        let txt = record.to_txt();
        assert_eq!(txt, "operators:*:1001:admin,operator");
        let parsed = GroupRecord::from_txt(&txt).unwrap();
        assert_eq!(record, parsed);
    }

    #[test]
    fn group_empty_members() {
        let record = GroupRecord {
            name: "empty".into(),
            gid: 9999,
            members: vec![],
        };
        let txt = record.to_txt();
        assert_eq!(txt, "empty:*:9999:");
        let parsed = GroupRecord::from_txt(&txt).unwrap();
        assert_eq!(parsed.members, Vec::<String>::new());
    }

    #[test]
    fn service_round_trip() {
        let record = ServiceRecord {
            host: "twingate.svc".into(),
            port: 443,
            protocol: "tcp".into(),
        };
        let txt = record.to_txt();
        assert_eq!(txt, "twingate.svc:443:tcp");
        let parsed = ServiceRecord::from_txt(&txt).unwrap();
        assert_eq!(record, parsed);
    }

    #[test]
    fn filsys_round_trip() {
        let record = FilsysRecord {
            fs_type: "nfs".into(),
            mount_path: "/home".into(),
            source: "nfsserver:/export".into(),
            mode: "rw".into(),
        };
        let txt = record.to_txt();
        assert_eq!(txt, "nfs /home nfsserver:/export rw");
        let parsed = FilsysRecord::from_txt(&txt).unwrap();
        assert_eq!(record, parsed);
    }

    #[test]
    fn hesiod_record_enum_round_trip() {
        let record = HesiodRecord::Service(ServiceRecord {
            host: "ipfs.svc".into(),
            port: 8080,
            protocol: "tcp".into(),
        });
        let txt = record.to_txt();
        let parsed = HesiodRecord::from_txt(MapType::Service, &txt).unwrap();
        assert_eq!(record, parsed);
    }

    #[test]
    fn map_type_parse() {
        assert_eq!("passwd".parse::<MapType>().unwrap(), MapType::Passwd);
        assert_eq!("GROUP".parse::<MapType>().unwrap(), MapType::Group);
        assert!("bogus".parse::<MapType>().is_err());
    }
}
