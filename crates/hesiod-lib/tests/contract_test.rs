// SPDX-License-Identifier: PMPL-1.0-or-later
//! Contract and reflexive tests for hesiod-lib.

use hesiod_lib::records::*;
use hesiod_lib::zone::HesiodZone;

/// Contract: All record types implement Display.
#[test]
fn contract_all_records_display() {
    let records = vec![
        HesiodRecord::Passwd(PasswdRecord {
            username: "test".into(),
            uid: 1000,
            gid: 1000,
            gecos: "Test".into(),
            home: "/home/test".into(),
            shell: "/bin/bash".into(),
        }),
        HesiodRecord::Group(GroupRecord {
            name: "test".into(),
            gid: 1000,
            members: vec!["user".into()],
        }),
        HesiodRecord::Service(ServiceRecord {
            host: "localhost".into(),
            port: 443,
            protocol: "tcp".into(),
        }),
        HesiodRecord::Filsys(FilsysRecord {
            fs_type: "nfs".into(),
            mount_path: "/home".into(),
            source: "nfs:/export".into(),
            mode: "rw".into(),
        }),
    ];

    for record in records {
        let _display = format!("{}", record);
        let _debug = format!("{:?}", record);
        // Both Display and Debug must work
    }
}

/// Contract: Zone domain is always retrievable and matches construction.
#[test]
fn contract_zone_domain_invariant() {
    let domains = vec![
        "test.local",
        "internal.example.com",
        "x",
        "very.deeply.nested.subdomain.example.org",
    ];

    for domain in domains {
        let zone = HesiodZone::new(domain, ".ns", ".example", 300);
        assert_eq!(zone.domain, domain);
    }
}

/// Contract: Zone TTL is always > 0 if set.
#[test]
fn contract_zone_ttl_positive() {
    let zone = HesiodZone::new("test.internal", ".ns", ".internal", 1);
    assert!(zone.ttl > 0);

    let zone2 = HesiodZone::new("test.internal", ".ns", ".internal", 86400);
    assert!(zone2.ttl > 0);
}

/// Contract: Lookup always returns Option, never panics.
#[test]
fn contract_zone_lookup_safe() {
    let mut zone = HesiodZone::new("test.internal", ".ns", ".internal", 300);
    zone.add_record("known", HesiodRecord::Service(ServiceRecord {
        host: "svc".into(),
        port: 443,
        protocol: "tcp".into(),
    }));

    // These should all return Option, never panic
    let _r1 = zone.lookup("known", MapType::Service);
    let _r2 = zone.lookup("unknown", MapType::Service);
    let _r3 = zone.lookup("known", MapType::Passwd);
    let _r4 = zone.lookup("", MapType::Group);
}

/// Contract: record_count always >= 0 and matches iteration.
#[test]
fn contract_record_count_accuracy() {
    let mut zone = HesiodZone::new("test.internal", ".ns", ".internal", 300);

    assert_eq!(zone.record_count(), 0);
    assert_eq!(zone.records().count(), 0);

    for i in 0..10 {
        zone.add_record(
            &format!("rec{}", i),
            HesiodRecord::Service(ServiceRecord {
                host: format!("svc{}", i).into(),
                port: 1000 + i as u16,
                protocol: "tcp".into(),
            }),
        );
    }

    assert_eq!(zone.record_count(), 10);
    assert_eq!(zone.records().count(), 10);
}

/// Contract: MapType parsing is bidirectional.
#[test]
fn contract_maptype_roundtrip() {
    let types = vec![MapType::Passwd, MapType::Group, MapType::Service, MapType::Filsys];

    for mt in types {
        let label = mt.label();
        let parsed: MapType = label.parse().expect("should parse");
        assert_eq!(mt, parsed);
    }
}

/// Contract: Record TXT serialization is deterministic.
#[test]
fn contract_txt_deterministic() {
    let record = PasswdRecord {
        username: "test".into(),
        uid: 1000,
        gid: 1000,
        gecos: "Test User".into(),
        home: "/home/test".into(),
        shell: "/bin/bash".into(),
    };

    let txt1 = record.to_txt();
    let txt2 = record.to_txt();
    assert_eq!(txt1, txt2);

    // Parse and re-serialize should be identical
    let parsed = PasswdRecord::from_txt(&txt1).unwrap();
    let txt3 = parsed.to_txt();
    assert_eq!(txt1, txt3);
}

/// Contract: Zone to_bind_zone() includes SOA and NS records.
#[test]
fn contract_bind_zone_structure() {
    let zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);
    let bind = zone.to_bind_zone();

    assert!(bind.contains("$ORIGIN"));
    assert!(bind.contains("@ IN SOA"));
    assert!(bind.contains("@ IN NS"));
    assert!(bind.contains("serial"));
}

/// Contract: Zone records() iterator is consistent.
#[test]
fn contract_zone_iteration_consistency() {
    let mut zone = HesiodZone::new("test.internal", ".ns", ".internal", 300);

    zone.add_record("a", HesiodRecord::Service(ServiceRecord {
        host: "svc".into(),
        port: 443,
        protocol: "tcp".into(),
    }));
    zone.add_record("b", HesiodRecord::Service(ServiceRecord {
        host: "svc2".into(),
        port: 8080,
        protocol: "tcp".into(),
    }));

    // Iterate twice, should get same records
    let iter1: Vec<_> = zone.records().map(|(name, _)| name).collect();
    let iter2: Vec<_> = zone.records().map(|(name, _)| name).collect();

    assert_eq!(iter1.len(), iter2.len());
    for name in iter1 {
        assert!(iter2.contains(&name));
    }
}

/// Contract: PasswdRecord field count is always 7 (split by colon).
#[test]
fn contract_passwd_field_count() {
    let record = PasswdRecord {
        username: "alice".into(),
        uid: 1000,
        gid: 1000,
        gecos: "Alice".into(),
        home: "/home/alice".into(),
        shell: "/bin/bash".into(),
    };

    let txt = record.to_txt();
    let fields: Vec<_> = txt.split(':').collect();
    assert_eq!(fields.len(), 7);
}

/// Contract: GroupRecord field count is always 4 (split by colon).
#[test]
fn contract_group_field_count() {
    let record = GroupRecord {
        name: "ops".into(),
        gid: 1001,
        members: vec!["alice".into(), "bob".into()],
    };

    let txt = record.to_txt();
    let fields: Vec<_> = txt.split(':').collect();
    assert_eq!(fields.len(), 4);
}

/// Contract: ServiceRecord field count is always 3 (split by colon).
#[test]
fn contract_service_field_count() {
    let record = ServiceRecord {
        host: "api.svc".into(),
        port: 8080,
        protocol: "tcp".into(),
    };

    let txt = record.to_txt();
    let fields: Vec<_> = txt.split(':').collect();
    assert_eq!(fields.len(), 3);
}

/// Contract: FilsysRecord field count is always 4 (split by space).
#[test]
fn contract_filsys_field_count() {
    let record = FilsysRecord {
        fs_type: "nfs".into(),
        mount_path: "/home".into(),
        source: "nfs:/export".into(),
        mode: "rw".into(),
    };

    let txt = record.to_txt();
    let fields: Vec<_> = txt.split(' ').collect();
    assert_eq!(fields.len(), 4);
}

/// Reflexive: record.key() corresponds to lookup key.
#[test]
fn reflexive_record_key_lookup() {
    let mut zone = HesiodZone::new("test.internal", ".ns", ".internal", 300);

    let passwd = PasswdRecord {
        username: "alice".into(),
        uid: 1000,
        gid: 1000,
        gecos: "Alice".into(),
        home: "/home/alice".into(),
        shell: "/bin/bash".into(),
    };

    let record = HesiodRecord::Passwd(passwd);
    let key = record.key();

    zone.add_record(key, record.clone());

    // Lookup with same key should find the record
    let found = zone.lookup(key, record.map_type()).expect("record should be found");
    assert_eq!(found.to_txt(), record.to_txt());
}

/// Reflexive: map_type() matches record variant.
#[test]
fn reflexive_record_type_matches_variant() {
    let records = vec![
        (
            HesiodRecord::Passwd(PasswdRecord {
                username: "test".into(),
                uid: 1000,
                gid: 1000,
                gecos: "".into(),
                home: "".into(),
                shell: "".into(),
            }),
            MapType::Passwd,
        ),
        (
            HesiodRecord::Group(GroupRecord {
                name: "test".into(),
                gid: 1000,
                members: vec![],
            }),
            MapType::Group,
        ),
        (
            HesiodRecord::Service(ServiceRecord {
                host: "test".into(),
                port: 443,
                protocol: "tcp".into(),
            }),
            MapType::Service,
        ),
        (
            HesiodRecord::Filsys(FilsysRecord {
                fs_type: "nfs".into(),
                mount_path: "test".into(),
                source: "".into(),
                mode: "rw".into(),
            }),
            MapType::Filsys,
        ),
    ];

    for (record, expected_type) in records {
        assert_eq!(record.map_type(), expected_type);
    }
}

/// Reflexive: Display matches to_txt().
#[test]
fn reflexive_display_equals_to_txt() {
    let records = vec![
        HesiodRecord::Passwd(PasswdRecord {
            username: "test".into(),
            uid: 1000,
            gid: 1000,
            gecos: "Test".into(),
            home: "/home/test".into(),
            shell: "/bin/bash".into(),
        }),
        HesiodRecord::Group(GroupRecord {
            name: "test".into(),
            gid: 1000,
            members: vec![],
        }),
        HesiodRecord::Service(ServiceRecord {
            host: "test".into(),
            port: 443,
            protocol: "tcp".into(),
        }),
    ];

    for record in records {
        let display = format!("{}", record);
        let to_txt = record.to_txt();
        assert_eq!(display, to_txt);
    }
}
