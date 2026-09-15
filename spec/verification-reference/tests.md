---
id: TM-001
title: "Verification reference amendment tests"
type: TestMatrix
relationships:
  - target: "ix://agent-ix/quire-specification/FR-017"
    type: references
  - target: "ix://agent-ix/quire-specification/FR-018"
    type: references
---
# Verification reference amendment tests

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
| --- | --- | --- | --- |
| FR-017 | FR-017-AC-1 | TC-001 | planned |
| FR-017 | FR-017-AC-2 | TC-002 | planned |
| FR-017 | FR-017-AC-3 | TC-002 | planned |
| FR-017 | FR-017-AC-4 | TC-002 | planned |
| FR-017 | FR-017-AC-5 | TC-002 | planned |
| FR-018 | FR-018-AC-1 | TC-003 | planned |
| FR-018 | FR-018-AC-2 | TC-003 | planned |
| FR-018 | FR-018-AC-3 | TC-003 | planned |
| FR-018 | FR-018-AC-4 | TC-003 | planned |
| FR-018 | FR-018-AC-5 | TC-004 | planned |
| FR-018 | FR-018-AC-6 | TC-003 | planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-001 | Six exact roles | Integration | P1 | FR-017-AC-1 | 🚧 Planned |
| TC-002 | Role/identity/version refusals | Property | P1 | FR-017-AC-2, FR-017-AC-3, FR-017-AC-4, FR-017-AC-5 | 🚧 Planned |
| TC-003 | Selected JCS and feature correspondence | Integration | P1 | FR-018-AC-1, FR-018-AC-2, FR-018-AC-3, FR-018-AC-4, FR-018-AC-6 | 🚧 Planned |
| TC-004 | Non-circular automated core | Integration | P1 | FR-018-AC-5 | 🚧 Planned |

## Integration Test Matrix

| Integration ID | Purpose | Target Project | Type | Test Cases | Status |
| --- | --- | --- | --- | --- | --- |
| INT-001 | Shared schema, JCS and strict reference correspondence | quire-verification | service | TC-001, TC-002, TC-003, TC-004 | 🚧 Planned |

The target is a local Rust library API; the catalog service classification does
not imply a hosted service. IT-003 specifies the actual file/API boundary.

## Evidence limits

IT-003 distinguishes structural qualification and existing JCS parity from the
later independent B shared reader/report run. Partial producer checks cannot
mark these complete integration cases done. No full-standard coverage is claimed.
