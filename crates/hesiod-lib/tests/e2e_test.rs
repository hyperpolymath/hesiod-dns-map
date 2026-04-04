// SPDX-License-Identifier: PMPL-1.0-or-later
//! End-to-end tests for hesiod-lib: config → zone → lookup pipeline.

use hesiod_lib::config::*;
use hesiod_lib::records::*;
use hesiod_lib::zone::HesiodZone;

#[test]
fn e2e_config_load_zone_lookup() {
    // Load config from JSON
    let json = r#"{
        "domain": "flatracoon.internal",
        "lhs": ".ns",
        "rhs": ".flatracoon.internal",
        "ttl": 300,
        "dns_port": 5353,
        "services": [
            {"name": "web", "host": "web.svc", "port": 443, "protocol": "tcp"},
            {"name": "api", "host": "api.svc", "port": 8080, "protocol": "tcp"}
        ],
        "users": [
            {"username": "alice", "uid": 1000, "gid": 1000, "gecos": "Alice", "home": "/home/alice", "shell": "/bin/bash"},
            {"username": "bob", "uid": 1001, "gid": 1000, "gecos": "Bob", "home": "/home/bob", "shell": "/bin/zsh"}
        ],
        "groups": [
            {"name": "ops", "gid": 1001, "members": ["alice"]},
            {"name": "dev", "gid": 1002, "members": ["alice", "bob"]}
        ]
    }"#;

    // Step 1: Parse config
    let config = HesiodConfig::from_json(json).expect("failed to parse config");
    assert_eq!(config.domain, "flatracoon.internal");
    assert_eq!(config.services.len(), 2);
    assert_eq!(config.users.len(), 2);
    assert_eq!(config.groups.len(), 2);

    // Step 2: Build zone from config
    let zone = HesiodZone::from_config(&config).expect("failed to build zone");
    assert_eq!(zone.domain, "flatracoon.internal");
    assert_eq!(zone.record_count(), 6); // 2 services + 2 users + 2 groups

    // Step 3: Lookup service
    let web_record = zone.lookup("web", MapType::Service).expect("web service not found");
    assert_eq!(web_record.to_txt(), "web.svc:443:tcp");

    // Step 4: Lookup user
    let alice = zone.lookup("alice", MapType::Passwd).expect("alice not found");
    let txt = alice.to_txt();
    assert!(txt.starts_with("alice:*:1000"));
    assert!(txt.contains("/home/alice"));

    // Step 5: Lookup group
    let ops = zone.lookup("ops", MapType::Group).expect("ops group not found");
    assert!(ops.to_txt().contains("1001"));
    assert!(ops.to_txt().contains("alice"));
}

#[test]
fn e2e_multi_record_lookup() {
    let config = HesiodConfig {
        domain: "test.internal".into(),
        lhs: ".ns".into(),
        rhs: ".test.internal".into(),
        ttl: 300,
        dns_port: 53,
        http_port: 8080,
        services: vec![
            ServiceEntry {
                name: "svc1".into(),
                host: "host1.local".into(),
                port: 443,
                protocol: "tcp".into(),
            },
            ServiceEntry {
                name: "svc2".into(),
                host: "host2.local".into(),
                port: 8080,
                protocol: "tcp".into(),
            },
        ],
        users: vec![],
        groups: vec![],
    };

    let zone = HesiodZone::from_config(&config).expect("failed to build zone");

    // Should be able to lookup multiple services
    let svc1 = zone.lookup("svc1", MapType::Service);
    let svc2 = zone.lookup("svc2", MapType::Service);

    assert!(svc1.is_some());
    assert!(svc2.is_some());
    assert_ne!(svc1.unwrap().to_txt(), svc2.unwrap().to_txt());
}

#[test]
fn e2e_missing_hostname_returns_none() {
    let config = HesiodConfig {
        domain: "test.internal".into(),
        lhs: ".ns".into(),
        rhs: ".test.internal".into(),
        ttl: 300,
        dns_port: 53,
        http_port: 8080,
        services: vec![ServiceEntry {
            name: "web".into(),
            host: "web.svc".into(),
            port: 443,
            protocol: "tcp".into(),
        }],
        users: vec![],
        groups: vec![],
    };

    let zone = HesiodZone::from_config(&config).expect("failed to build zone");

    // Lookup non-existent service
    let result = zone.lookup("nonexistent", MapType::Service);
    assert!(result.is_none(), "non-existent hostname should return None, not panic");
}

#[test]
fn e2e_zone_bind_output_contains_all_records() {
    let config = HesiodConfig {
        domain: "example.com".into(),
        lhs: ".ns".into(),
        rhs: ".example.com".into(),
        ttl: 600,
        dns_port: 53,
        http_port: 8080,
        services: vec![ServiceEntry {
            name: "www".into(),
            host: "web.example.com".into(),
            port: 80,
            protocol: "tcp".into(),
        }],
        users: vec![UserEntry {
            username: "admin".into(),
            uid: 1000,
            gid: 1000,
            gecos: "Administrator".into(),
            home: "/root".into(),
            shell: "/bin/bash".into(),
        }],
        groups: vec![GroupEntry {
            name: "admin".into(),
            gid: 1000,
            members: vec!["admin".into()],
        }],
    };

    let zone = HesiodZone::from_config(&config).expect("failed to build zone");
    let bind_output = zone.to_bind_zone();

    // Verify BIND zone file structure
    assert!(bind_output.contains("$ORIGIN .example.com."));
    assert!(bind_output.contains("@ IN SOA"));
    assert!(bind_output.contains("@ IN NS"));
    assert!(bind_output.contains("HS TXT"));
    assert!(bind_output.contains("Service records"));
    assert!(bind_output.contains("Passwd records"));
    assert!(bind_output.contains("Group records"));

    // Verify records are in zone
    assert!(bind_output.contains("www.service.ns"));
    assert!(bind_output.contains("admin.passwd.ns"));
    assert!(bind_output.contains("admin.group.ns"));
}

#[test]
fn e2e_full_record_lifecycle() {
    let mut zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);

    // Add service
    zone.add_record(
        "web",
        HesiodRecord::Service(ServiceRecord {
            host: "web.svc".into(),
            port: 443,
            protocol: "https".into(),
        }),
    );

    // Add user
    zone.add_record(
        "alice",
        HesiodRecord::Passwd(PasswdRecord {
            username: "alice".into(),
            uid: 1000,
            gid: 1000,
            gecos: "Alice".into(),
            home: "/home/alice".into(),
            shell: "/bin/bash".into(),
        }),
    );

    // Add group
    zone.add_record(
        "admin",
        HesiodRecord::Group(GroupRecord {
            name: "admin".into(),
            gid: 1000,
            members: vec!["alice".into()],
        }),
    );

    // Add filsys
    zone.add_record(
        "home",
        HesiodRecord::Filsys(FilsysRecord {
            fs_type: "nfs".into(),
            mount_path: "/home".into(),
            source: "nfs.local:/export".into(),
            mode: "rw".into(),
        }),
    );

    assert_eq!(zone.record_count(), 4);

    // Verify all records are retrievable
    assert!(zone.lookup("web", MapType::Service).is_some());
    assert!(zone.lookup("alice", MapType::Passwd).is_some());
    assert!(zone.lookup("admin", MapType::Group).is_some());
    assert!(zone.lookup("home", MapType::Filsys).is_some());

    // Verify correct map type separation
    assert!(zone.lookup("web", MapType::Passwd).is_none());
    assert!(zone.lookup("alice", MapType::Service).is_none());
}

#[test]
fn e2e_zone_serialization() {
    let config = HesiodConfig {
        domain: "test.internal".into(),
        lhs: ".ns".into(),
        rhs: ".test.internal".into(),
        ttl: 300,
        dns_port: 53,
        http_port: 8080,
        services: vec![ServiceEntry {
            name: "db".into(),
            host: "postgres.svc".into(),
            port: 5432,
            protocol: "tcp".into(),
        }],
        users: vec![],
        groups: vec![],
    };

    let zone = HesiodZone::from_config(&config).expect("failed to build zone");

    // Serialize to JSON and back (if serialization is supported)
    let record = zone.lookup("db", MapType::Service).unwrap();
    let txt = record.to_txt();
    assert_eq!(txt, "postgres.svc:5432:tcp");

    // Parse back from TXT
    let parsed = HesiodRecord::from_txt(MapType::Service, &txt).expect("failed to parse");
    assert_eq!(parsed.to_txt(), txt);
}

#[test]
fn e2e_default_values_applied() {
    // Config with minimal fields should get defaults
    let json = r#"{
        "domain": "minimal.internal",
        "lhs": ".ns",
        "rhs": ".minimal.internal"
    }"#;

    let config = HesiodConfig::from_json(json).expect("failed to parse");

    // Defaults should be applied
    assert_eq!(config.ttl, 300);
    assert_eq!(config.dns_port, 53);
    assert_eq!(config.http_port, 8080);
    assert!(config.services.is_empty());
    assert!(config.users.is_empty());
    assert!(config.groups.is_empty());

    let zone = HesiodZone::from_config(&config).expect("failed to build zone");
    assert_eq!(zone.record_count(), 0);
}

#[test]
fn e2e_special_characters_in_gecos() {
    let config = HesiodConfig {
        domain: "test.internal".into(),
        lhs: ".ns".into(),
        rhs: ".test.internal".into(),
        ttl: 300,
        dns_port: 53,
        http_port: 8080,
        services: vec![],
        users: vec![UserEntry {
            username: "bob".into(),
            uid: 1001,
            gid: 1001,
            gecos: "Bob Smith, Room 123".into(),
            home: "/home/bob".into(),
            shell: "/bin/bash".into(),
        }],
        groups: vec![],
    };

    let zone = HesiodZone::from_config(&config).expect("failed to build zone");
    let bob = zone.lookup("bob", MapType::Passwd).expect("bob not found");
    let txt = bob.to_txt();

    assert!(txt.contains("Bob Smith, Room 123"));
}

#[test]
fn e2e_zone_iteration() {
    let mut zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);

    zone.add_record("web", HesiodRecord::Service(ServiceRecord {
        host: "web.svc".into(),
        port: 443,
        protocol: "tcp".into(),
    }));

    zone.add_record("api", HesiodRecord::Service(ServiceRecord {
        host: "api.svc".into(),
        port: 8080,
        protocol: "tcp".into(),
    }));

    let records: Vec<_> = zone.records().collect();
    assert_eq!(records.len(), 2);

    // Verify we can iterate over all records
    let names: Vec<_> = records.iter().map(|(name, _)| *name).collect();
    assert!(names.contains(&"web"));
    assert!(names.contains(&"api"));
}
