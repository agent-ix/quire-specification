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

| Functional Req | Acceptance Criteria | Test Cases | Status |
| --- | --- | --- | --- |
| FR-017 | FR-017-AC-1 | TC-001 |  |
| FR-017 | FR-017-AC-2 | TC-002 |  |
| FR-017 | FR-017-AC-3 | TC-002 |  |
| FR-017 | FR-017-AC-4 | TC-002 |  |
| FR-017 | FR-017-AC-5 | TC-002 |  |
| FR-018 | FR-018-AC-1 | TC-003 |  |
| FR-018 | FR-018-AC-2 | TC-003 |  |
| FR-018 | FR-018-AC-3 | TC-003 |  |
| FR-018 | FR-018-AC-4 | TC-003 |  |
| FR-018 | FR-018-AC-5 | TC-004 |  |
| FR-018 | FR-018-AC-6 | TC-003 |  |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-001 | Six exact roles | Integration | P1 | FR-017-AC-1 | 🚧 |
| TC-002 | Role/identity/version refusals | Property | P1 | FR-017-AC-2, FR-017-AC-3, FR-017-AC-4, FR-017-AC-5 | 🚧 |
| TC-003 | Selected JCS and feature correspondence | Integration | P1 | FR-018-AC-1, FR-018-AC-2, FR-018-AC-3, FR-018-AC-4, FR-018-AC-6 | 🚧 |
| TC-004 | Non-circular automated core | Integration | P1 | FR-018-AC-5 | 🚧 |

## Integration Test Matrix

| Integration ID | Purpose | Target Project | Type | Test Cases |
| --- | --- | --- | --- | --- |
| INT-001 | Shared schema, JCS and strict reference correspondence | quire-verification | service | TC-001, TC-002, TC-003, TC-004 |

The target is a local Rust library API; the catalog service classification does
not imply a hosted service. IT-003 specifies the actual file/API boundary.
