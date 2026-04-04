// SPDX-License-Identifier: PMPL-1.0-or-later
//! Criterion benchmarks for hesiod-lib DNS operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use hesiod_lib::config::*;
use hesiod_lib::records::*;
use hesiod_lib::zone::HesiodZone;

/// Benchmark: Zone lookup performance for service records.
fn bench_zone_service_lookup(c: &mut Criterion) {
    let mut zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);

    // Add 100 service records
    for i in 0..100 {
        zone.add_record(
            &format!("service{}", i),
            HesiodRecord::Service(ServiceRecord {
                host: format!("host{}.svc", i).into(),
                port: 1000 + (i as u16),
                protocol: "tcp".into(),
            }),
        );
    }

    c.bench_function("zone_lookup_existing_service", |b| {
        b.iter(|| {
            zone.lookup(black_box("service50"), black_box(MapType::Service))
        })
    });

    c.bench_function("zone_lookup_missing_service", |b| {
        b.iter(|| {
            zone.lookup(black_box("nonexistent"), black_box(MapType::Service))
        })
    });
}

/// Benchmark: Zone lookup performance for passwd records.
fn bench_zone_passwd_lookup(c: &mut Criterion) {
    let mut zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);

    // Add 50 user records
    for i in 0..50 {
        zone.add_record(
            &format!("user{}", i),
            HesiodRecord::Passwd(PasswdRecord {
                username: format!("user{}", i).into(),
                uid: 1000 + (i as u32),
                gid: 1000,
                gecos: format!("User {}", i).into(),
                home: format!("/home/user{}", i).into(),
                shell: "/bin/bash".into(),
            }),
        );
    }

    c.bench_function("zone_lookup_existing_passwd", |b| {
        b.iter(|| {
            zone.lookup(black_box("user25"), black_box(MapType::Passwd))
        })
    });

    c.bench_function("zone_lookup_missing_passwd", |b| {
        b.iter(|| {
            zone.lookup(black_box("unknown_user"), black_box(MapType::Passwd))
        })
    });
}

/// Benchmark: Record serialization (to_txt) performance.
fn bench_record_serialization(c: &mut Criterion) {
    let passwd = PasswdRecord {
        username: "testuser".into(),
        uid: 1000,
        gid: 1000,
        gecos: "Test User Account".into(),
        home: "/home/testuser".into(),
        shell: "/bin/bash".into(),
    };

    let service = ServiceRecord {
        host: "api.service.local".into(),
        port: 8080,
        protocol: "tcp".into(),
    };

    let group = GroupRecord {
        name: "developers".into(),
        gid: 1000,
        members: vec!["alice".into(), "bob".into(), "charlie".into()],
    };

    let filsys = FilsysRecord {
        fs_type: "nfs".into(),
        mount_path: "/home".into(),
        source: "nfsserver.local:/export/home".into(),
        mode: "rw".into(),
    };

    c.bench_function("serialize_passwd_to_txt", |b| {
        b.iter(|| passwd.clone().to_txt())
    });

    c.bench_function("serialize_service_to_txt", |b| {
        b.iter(|| service.clone().to_txt())
    });

    c.bench_function("serialize_group_to_txt", |b| {
        b.iter(|| group.clone().to_txt())
    });

    c.bench_function("serialize_filsys_to_txt", |b| {
        b.iter(|| filsys.clone().to_txt())
    });
}

/// Benchmark: Record parsing (from_txt) performance.
fn bench_record_parsing(c: &mut Criterion) {
    let passwd_txt = "testuser:*:1000:1000:Test User Account:/home/testuser:/bin/bash";
    let service_txt = "api.service.local:8080:tcp";
    let group_txt = "developers:*:1000:alice,bob,charlie";
    let filsys_txt = "nfs /home nfsserver.local:/export/home rw";

    c.bench_function("parse_passwd_from_txt", |b| {
        b.iter(|| PasswdRecord::from_txt(black_box(passwd_txt)))
    });

    c.bench_function("parse_service_from_txt", |b| {
        b.iter(|| ServiceRecord::from_txt(black_box(service_txt)))
    });

    c.bench_function("parse_group_from_txt", |b| {
        b.iter(|| GroupRecord::from_txt(black_box(group_txt)))
    });

    c.bench_function("parse_filsys_from_txt", |b| {
        b.iter(|| FilsysRecord::from_txt(black_box(filsys_txt)))
    });
}

/// Benchmark: Config parsing from JSON.
fn bench_config_parsing(c: &mut Criterion) {
    let json_small = r#"{
        "domain": "test.internal",
        "lhs": ".ns",
        "rhs": ".test.internal",
        "ttl": 300
    }"#;

    let json_medium = r#"{
        "domain": "flatracoon.internal",
        "lhs": ".ns",
        "rhs": ".flatracoon.internal",
        "ttl": 600,
        "dns_port": 5353,
        "http_port": 9090,
        "services": [
            {"name": "web", "host": "web.svc", "port": 443, "protocol": "tcp"},
            {"name": "api", "host": "api.svc", "port": 8080, "protocol": "tcp"},
            {"name": "db", "host": "postgres.svc", "port": 5432, "protocol": "tcp"}
        ],
        "users": [
            {"username": "alice", "uid": 1000, "gid": 1000, "gecos": "Alice", "home": "/home/alice", "shell": "/bin/bash"},
            {"username": "bob", "uid": 1001, "gid": 1000, "gecos": "Bob", "home": "/home/bob", "shell": "/bin/bash"}
        ],
        "groups": [
            {"name": "ops", "gid": 1001, "members": ["alice"]},
            {"name": "dev", "gid": 1002, "members": ["alice", "bob"]}
        ]
    }"#;

    c.bench_function("parse_config_small", |b| {
        b.iter(|| HesiodConfig::from_json(black_box(json_small)))
    });

    c.bench_function("parse_config_medium", |b| {
        b.iter(|| HesiodConfig::from_json(black_box(json_medium)))
    });
}

/// Benchmark: Zone construction from config.
fn bench_zone_construction(c: &mut Criterion) {
    let json = r#"{
        "domain": "test.internal",
        "lhs": ".ns",
        "rhs": ".test.internal",
        "ttl": 300,
        "services": [
            {"name": "svc0", "host": "host0.svc", "port": 443, "protocol": "tcp"},
            {"name": "svc1", "host": "host1.svc", "port": 8080, "protocol": "tcp"},
            {"name": "svc2", "host": "host2.svc", "port": 5432, "protocol": "tcp"}
        ],
        "users": [
            {"username": "user0", "uid": 1000, "gid": 1000, "gecos": "User 0", "home": "/home/user0", "shell": "/bin/bash"},
            {"username": "user1", "uid": 1001, "gid": 1000, "gecos": "User 1", "home": "/home/user1", "shell": "/bin/bash"}
        ],
        "groups": [
            {"name": "group0", "gid": 1000, "members": ["user0"]}
        ]
    }"#;

    c.bench_function("zone_construction_from_config", |b| {
        b.iter(|| {
            let config = HesiodConfig::from_json(black_box(json))
                .expect("config parse failed");
            HesiodZone::from_config(black_box(&config))
                .expect("zone construction failed")
        })
    });
}

/// Benchmark: Zone BIND output generation.
fn bench_zone_bind_output(c: &mut Criterion) {
    let mut zone = HesiodZone::new("example.com", ".ns", ".example.com", 300);

    // Add various records
    for i in 0..10 {
        zone.add_record(
            &format!("service{}", i),
            HesiodRecord::Service(ServiceRecord {
                host: format!("host{}.svc", i).into(),
                port: 1000 + (i as u16),
                protocol: "tcp".into(),
            }),
        );
    }

    for i in 0..5 {
        zone.add_record(
            &format!("user{}", i),
            HesiodRecord::Passwd(PasswdRecord {
                username: format!("user{}", i).into(),
                uid: 1000 + (i as u32),
                gid: 1000,
                gecos: format!("User {}", i).into(),
                home: format!("/home/user{}", i).into(),
                shell: "/bin/bash".into(),
            }),
        );
    }

    c.bench_function("zone_to_bind_zone", |b| {
        b.iter(|| zone.to_bind_zone())
    });
}

/// Benchmark: Zone iteration performance.
fn bench_zone_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("zone_iteration");

    for record_count in [10, 100, 1000].iter() {
        let mut zone = HesiodZone::new("test.internal", ".ns", ".test.internal", 300);

        for i in 0..*record_count {
            zone.add_record(
                &format!("record{}", i),
                HesiodRecord::Service(ServiceRecord {
                    host: format!("host{}.svc", i).into(),
                    port: 1000 + (i as u16 % 64000),
                    protocol: "tcp".into(),
                }),
            );
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(record_count),
            record_count,
            |b, _| {
                b.iter(|| {
                    zone.records().count()
                })
            },
        );
    }

    group.finish();
}

/// Benchmark: MapType parsing performance.
fn bench_maptype_parsing(c: &mut Criterion) {
    let map_types = vec!["passwd", "group", "service", "filsys"];

    c.bench_function("parse_maptype_passwd", |b| {
        b.iter(|| "passwd".parse::<MapType>())
    });

    c.bench_function("parse_maptype_group", |b| {
        b.iter(|| "group".parse::<MapType>())
    });

    c.bench_function("parse_maptype_service", |b| {
        b.iter(|| "service".parse::<MapType>())
    });

    c.bench_function("parse_maptype_filsys", |b| {
        b.iter(|| "filsys".parse::<MapType>())
    });
}

criterion_group!(
    benches,
    bench_zone_service_lookup,
    bench_zone_passwd_lookup,
    bench_record_serialization,
    bench_record_parsing,
    bench_config_parsing,
    bench_zone_construction,
    bench_zone_bind_output,
    bench_zone_iteration,
    bench_maptype_parsing,
);
criterion_main!(benches);
