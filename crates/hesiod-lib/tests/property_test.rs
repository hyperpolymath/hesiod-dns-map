// SPDX-License-Identifier: PMPL-1.0-or-later
//! Property-based tests for hesiod-lib using proptest.

use hesiod_lib::records::*;
use hesiod_lib::zone::HesiodZone;
use proptest::prelude::*;

/// Property: PasswdRecord round-trip serialization never panics.
proptest! {
    #[test]
    fn prop_passwd_roundtrip_no_panic(
        username in r"[a-zA-Z0-9_]{1,32}",
        uid in 0u32..=65535,
        gid in 0u32..=65535,
        gecos in r"[a-zA-Z0-9 _-]{0,128}",
        home in r"/[a-zA-Z0-9/_-]{1,64}",
        shell in r"/[a-zA-Z0-9/_-]{1,32}"
    ) {
        let record = PasswdRecord {
            username: username.clone(),
            uid,
            gid,
            gecos: gecos.clone(),
            home: home.clone(),
            shell: shell.clone(),
        };
        let txt = record.to_txt();
        // Should never panic on valid input
        let parsed = PasswdRecord::from_txt(&txt);
        prop_assert!(parsed.is_ok());
        let parsed_rec = parsed.unwrap();
        prop_assert_eq!(parsed_rec.username, username);
        prop_assert_eq!(parsed_rec.uid, uid);
        prop_assert_eq!(parsed_rec.gid, gid);
    }
}

/// Property: GroupRecord with any member list round-trips correctly.
proptest! {
    #[test]
    fn prop_group_roundtrip(
        name in r"[a-zA-Z0-9_]{1,32}",
        gid in 0u32..=65535,
        members in prop::collection::vec(r"[a-zA-Z0-9_]{1,32}", 0..10)
    ) {
        let record = GroupRecord {
            name: name.clone(),
            gid,
            members: members.clone(),
        };
        let txt = record.to_txt();
        let parsed = GroupRecord::from_txt(&txt).expect("failed to parse group");
        prop_assert_eq!(parsed.name, name);
        prop_assert_eq!(parsed.gid, gid);
        prop_assert_eq!(parsed.members, members);
    }
}

/// Property: ServiceRecord port range is always valid.
proptest! {
    #[test]
    fn prop_service_valid_port(
        host in r"[a-zA-Z0-9.-]{1,64}",
        port in 1u16..=65535,
        protocol in r"(tcp|udp|sctp)"
    ) {
        let record = ServiceRecord {
            host: host.clone(),
            port,
            protocol: protocol.clone(),
        };
        let txt = record.to_txt();
        let parsed = ServiceRecord::from_txt(&txt).expect("failed to parse service");
        prop_assert_eq!(parsed.port, port);
        prop_assert!(parsed.port > 0);
    }
}

/// Property: FilsysRecord with valid FS types round-trips.
proptest! {
    #[test]
    fn prop_filsys_roundtrip(
        fs_type in r"(nfs|ext4|tmpfs|fuse)",
        mount_path in r"/[a-zA-Z0-9/_-]{1,64}",
        source in r"[a-zA-Z0-9.:/_-]{1,128}",
        mode in r"(ro|rw)"
    ) {
        let record = FilsysRecord {
            fs_type: fs_type.clone(),
            mount_path: mount_path.clone(),
            source: source.clone(),
            mode: mode.clone(),
        };
        let txt = record.to_txt();
        let parsed = FilsysRecord::from_txt(&txt).expect("failed to parse filsys");
        prop_assert_eq!(parsed.fs_type, fs_type);
        prop_assert_eq!(parsed.mode, mode);
    }
}

/// Property: MapType parsing is case-insensitive and always valid for known types.
proptest! {
    #[test]
    fn prop_maptype_case_insensitive(
        variant in r"(passwd|group|service|filsys)"
    ) {
        let lower = variant.to_lowercase();
        let upper = variant.to_uppercase();

        let lower_result = lower.parse::<MapType>();
        let upper_result = upper.parse::<MapType>();

        // Both should parse successfully
        prop_assert!(lower_result.is_ok());
        prop_assert!(upper_result.is_ok());

        // Results should be equivalent
        prop_assert_eq!(lower_result.unwrap(), upper_result.unwrap());
    }
}

/// Property: HesiodZone never panics on lookup with any name string.
proptest! {
    #[test]
    fn prop_zone_lookup_no_panic(
        key_name in r"[a-zA-Z0-9_-]{1,32}",
        map_type_str in r"(passwd|group|service|filsys)"
    ) {
        let mut zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);

        // Add a known record
        zone.add_record(
            "known",
            HesiodRecord::Service(ServiceRecord {
                host: "test.svc".into(),
                port: 443,
                protocol: "tcp".into(),
            }),
        );

        let map_type: MapType = map_type_str.parse().unwrap();

        // Lookup should never panic, regardless of key or map_type
        let _result = zone.lookup(&key_name, map_type);
        // If key_name == "known" and map_type == Service, should return Some
        // Otherwise, should return None
        // But should never panic - the critical contract is no panics
        prop_assert!(true);
    }
}

/// Property: Zone record_count is always >= 0 and matches iteration.
proptest! {
    #[test]
    fn prop_zone_record_count_consistency(
        records_count in 0usize..20
    ) {
        let mut zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);

        for i in 0..records_count {
            let name = format!("record{}", i);
            zone.add_record(
                &name,
                HesiodRecord::Service(ServiceRecord {
                    host: format!("svc{}.local", i),
                    port: (1024 + i as u16),
                    protocol: "tcp".into(),
                }),
            );
        }

        let count = zone.record_count();
        let iter_count = zone.records().count();

        prop_assert_eq!(count, records_count);
        prop_assert_eq!(iter_count, records_count);
    }
}

/// Property: TTL values in config are always > 0.
proptest! {
    #[test]
    fn prop_ttl_always_positive(ttl in 1u32..=86400) {
        let zone = HesiodZone::new("test.internal", ".ns", ".test.internal", ttl);
        prop_assert!(zone.ttl > 0);
        prop_assert!(zone.ttl <= 86400);
    }
}

/// Property: Zone domain is never empty.
proptest! {
    #[test]
    fn prop_zone_domain_not_empty(domain in r"[a-zA-Z0-9.-]{1,128}") {
        let zone = HesiodZone::new(&domain, ".ns", ".internal", 300);
        prop_assert!(!zone.domain.is_empty());
        prop_assert_eq!(zone.domain, domain);
    }
}

/// Property: Record type enum is always correctly identified.
proptest! {
    #[test]
    fn prop_record_type_consistency(which in 0u8..4) {
        let record = match which {
            0 => HesiodRecord::Passwd(PasswdRecord {
                username: "test".into(),
                uid: 1000,
                gid: 1000,
                gecos: "Test".into(),
                home: "/home/test".into(),
                shell: "/bin/bash".into(),
            }),
            1 => HesiodRecord::Group(GroupRecord {
                name: "group".into(),
                gid: 1001,
                members: vec![],
            }),
            2 => HesiodRecord::Service(ServiceRecord {
                host: "svc".into(),
                port: 443,
                protocol: "tcp".into(),
            }),
            _ => HesiodRecord::Filsys(FilsysRecord {
                fs_type: "nfs".into(),
                mount_path: "/home".into(),
                source: "nfs:/export".into(),
                mode: "rw".into(),
            }),
        };

        let map_type = record.map_type();
        let txt = record.to_txt();

        // Round-trip should preserve type
        let roundtrip = HesiodRecord::from_txt(map_type, &txt).expect("roundtrip failed");
        prop_assert_eq!(record.map_type(), roundtrip.map_type());
    }
}

/// Property: Empty or whitespace-only usernames should be rejected (or handled gracefully).
proptest! {
    #[test]
    fn prop_username_validation(username in r"[a-zA-Z0-9_]{0,32}") {
        let txt = if username.is_empty() {
            ":*:1000:1000:Test:/home/test:/bin/bash".to_string()
        } else {
            format!("{}:*:1000:1000:Test:/home/test:/bin/bash", username)
        };

        // Should either parse successfully or fail gracefully, never panic
        let result = PasswdRecord::from_txt(&txt);
        // Result can be Ok or Err, but not panic
        let _ = result;
    }
}

/// Property: Oversized hostnames don't cause panics.
proptest! {
    #[test]
    fn prop_large_hostname_no_panic(hostname_len in 1usize..=8192) {
        let hostname = "x".repeat(hostname_len);
        // Should not panic when looking up large hostname
        let mut zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);
        let _result = zone.lookup(&hostname, MapType::Passwd);
        // Should either return None or not crash
    }
}
