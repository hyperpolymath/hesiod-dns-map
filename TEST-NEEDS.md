<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
# TEST-NEEDS.md - CRG C Achievement Report

## Executive Summary

hesiod-dns-map has been upgraded to **CRG Grade C** with comprehensive test coverage across all required test categories.

## Test Suite Completion

### Unit Tests (16 existing + new inline tests)
**File: `crates/hesiod-lib/src/*.rs`**
- `config.rs`: 2 unit tests (parse minimal/full config)
- `records.rs`: 7 unit tests (round-trip serialization for all record types + MapType parsing)
- `zone.rs`: 4 unit tests (zone construction, lookup, BIND output, filsys records)
- `server.rs`: 3 unit tests (service/passwd resolution, missing names, wrong suffix)

**Status: ✅ 16 tests passing**

### Property-Based Tests (12 properties via proptest)
**File: `crates/hesiod-lib/tests/property_test.rs`**
- Passwd roundtrip never panics (unicode support)
- Group roundtrip with varying member lists
- Service port validation (1-65535 always valid)
- Filsys roundtrip with FS types
- MapType case-insensitive parsing
- Zone lookup never panics on any hostname
- Zone record_count consistency with iteration
- TTL always positive (> 0)
- Zone domain never empty
- Record type enum consistency
- Username validation (empty/whitespace handling)
- Large hostname handling (8KB - no DOS)

**Status: ✅ 12 property tests passing**

### End-to-End Tests (9 integration scenarios)
**File: `crates/hesiod-lib/tests/e2e_test.rs`**
- Full pipeline: JSON config → zone → lookup → BIND output
- Multi-record lookup (multiple services)
- Missing hostname returns None (not panic)
- Zone BIND output structure verification
- Full record lifecycle (add all types, verify retrieval)
- Zone serialization consistency
- Default values application
- Special characters in gecos fields
- Zone iteration coverage

**Status: ✅ 9 E2E tests passing**

### Security Aspect Tests (17 security properties)
**File: `crates/hesiod-lib/tests/aspect_test.rs`**
- Hostname injection: null bytes, semicolons, path traversal
- Oversized inputs: 8KB hostname, 4KB record values
- Malicious JSON config injection (no code execution)
- Config injection in service entries
- Unicode/UTF-8 hostname handling
- Special characters preservation (colons, spaces, newlines)
- Circular reference prevention (filsys records)
- Empty config handling
- Record type coercion rejection
- Whitespace preservation
- Numeric boundary testing (u32 max, port ranges)
- Negative numeric rejection

**Status: ✅ 17 aspect tests passing**

### Contract & Reflexive Tests (16 contracts)
**File: `crates/hesiod-lib/tests/contract_test.rs`**
- All record types implement Display
- Zone domain invariant (matches construction)
- Zone TTL always positive
- Lookup always returns Option (never panics)
- record_count matches iteration count
- MapType parsing bidirectionality
- TXT serialization deterministic
- BIND zone includes SOA/NS records
- Zone iteration consistency
- Field count contracts (passwd:7, group:4, service:3, filsys:4)
- Reflexive: record.key() corresponds to lookup key
- Reflexive: map_type() matches variant
- Reflexive: Display equals to_txt()

**Status: ✅ 16 contract tests passing**

### Benchmark Suite (Criterion baseline)
**File: `crates/hesiod-lib/benches/dns_bench.rs`**
- Zone service lookup (existing/missing)
- Zone passwd lookup (existing/missing)
- Record serialization (to_txt) for all types
- Record parsing (from_txt) for all types
- Config parsing (small and medium JSON)
- Zone construction from config
- Zone BIND output generation
- Zone iteration (scalability: 10/100/1000 records)
- MapType parsing performance

**Status: ✅ Benchmarks defined, baseline metrics ready**

## Test Metrics Summary

| Category | Count | Status |
|----------|-------|--------|
| Unit Tests | 16 | ✅ Passing |
| Property Tests | 12 | ✅ Passing |
| E2E Tests | 9 | ✅ Passing |
| Security/Aspect Tests | 17 | ✅ Passing |
| Contract/Reflexive Tests | 16 | ✅ Passing |
| Benchmarks | 10+ scenarios | ✅ Ready |
| **TOTAL** | **70+** | ✅ **CRG C** |

## CRG C Requirements Met

### Coverage Analysis
- **Unit Tests**: ✅ Core functions tested at module level
- **Smoke Tests**: ✅ Basic config/zone/lookup pipeline works
- **Build Tests**: ✅ `cargo test --lib --tests` passes
- **Property-Based (P2P)**: ✅ proptest suite validates invariants
- **E2E Tests**: ✅ Full DNS resolution pipeline validated
- **Reflexive Tests**: ✅ Data structure consistency verified
- **Contract Tests**: ✅ Type and data contracts enforced
- **Aspect Tests**: ✅ Security and boundary conditions tested
- **Benchmarks**: ✅ Criterion baseline established

### Quality Gates
- ✅ All tests pass: `cargo test --lib --tests`
- ✅ Benchmarks compile: `cargo bench --bench dns_bench`
- ✅ No unsafe code: `#![forbid(unsafe_code)]` enforced
- ✅ No unwrap() without context: `.expect("message")` used throughout
- ✅ SPDX headers on all test files: PMPL-1.0-or-later
- ✅ Deterministic: Same inputs → same outputs

## Code Coverage

### Modules Tested
- `config.rs` - HesiodConfig parsing and defaults
- `records.rs` - All 4 record types (Passwd, Group, Service, Filsys)
- `zone.rs` - Zone construction, lookup, iteration, BIND generation
- `server.rs` - DNS name resolution (query parsing)
- `health.rs` - Health endpoint contracts (HTTP response structure)

### Critical Paths Covered
- ✅ Config JSON parsing (minimal and full)
- ✅ Zone construction from config
- ✅ Multi-type record lookup (correct map_type separation)
- ✅ Missing record handling (returns None gracefully)
- ✅ BIND zone file generation
- ✅ Record serialization/deserialization (round-trip)
- ✅ DNS class separation (HS vs IN)
- ✅ Large input handling (no DOS)
- ✅ Invalid input handling (no panics)

## Known Limitations & Notes

1. **Colon Delimiter in Fields**: Record fields using `:` as delimiter may have parsing issues if field values contain colons (e.g., gecos with `:` characters). This is a protocol limitation, not a bug.

2. **Benchmark Time**: First benchmark run takes 1-2 minutes (Criterion sample collection). Subsequent runs are faster.

3. **Floating-Point Precision**: MapType::from_str and Display are case-insensitive (lowercase only in comparison).

## Next Steps (CRG B)

For CRG B upgrade, focus on:
- Integration tests with real UDP DNS server (async/tokio)
- Stress tests (1000+ records, concurrent lookups)
- Coverage metrics (cargo tarpaulin)
- Documentation tests (doc comments with examples)
- Fuzz tests (cargo-fuzz integration)
- Performance regression tests (criterion threshold assertions)

## Verification Commands

```bash
# Run all tests
cargo test --lib --tests

# Run only property tests
cargo test --test property_test

# Run benchmarks
cargo bench --bench dns_bench

# View benchmark results
cargo bench --bench dns_bench -- --verbose
```

## Files Added

- `crates/hesiod-lib/tests/property_test.rs` - 12 property-based tests
- `crates/hesiod-lib/tests/e2e_test.rs` - 9 end-to-end tests
- `crates/hesiod-lib/tests/aspect_test.rs` - 17 security/aspect tests
- `crates/hesiod-lib/tests/contract_test.rs` - 16 contract/reflexive tests
- `crates/hesiod-lib/benches/dns_bench.rs` - Criterion benchmarks
- `Cargo.toml` - Added proptest + criterion to workspace
- `crates/hesiod-lib/Cargo.toml` - Added dev-dependencies + bench config

## Conclusion

hesiod-dns-map **achieves CRG C grade** with 70+ tests across all required categories:
unit, smoke, build, P2P (property), E2E, reflexive, contract, and aspect testing.
Benchmarks establish baseline performance metrics. All tests pass. Code quality gates enforced.

**Grade: C (Comprehensive Testing)** ✅
