---
id: TM-006
title: "Native temporal semantics and TL correspondence acceptance"
type: TestMatrix
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-091
    type: references
  - target: ix://agent-ix/quire-specification/FR-092
    type: references
  - target: ix://agent-ix/quire-specification/FR-093
    type: references
  - target: ix://agent-ix/quire-specification/FR-094
    type: references
  - target: ix://agent-ix/quire-specification/FR-095
    type: references
  - target: ix://agent-ix/quire-specification/NFR-040
    type: references
  - target: ix://agent-ix/quire-specification/FR-295
    type: references
  - target: ix://agent-ix/quire-specification/FR-289
    type: references
  - target: ix://agent-ix/quire-specification/FR-293
    type: references
---
## Overview

Draft Agent E family matrix. Every case is planned and unimplemented. Existing
TL tests establish only their own historical profiles; they are inspected input,
not coverage of native Quire or the bridge. FS01/FS02, integrated grammar and
the coherent base-plus-seven review remain acceptance prerequisites.

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
| --- | --- | --- | --- |
| FR-090 | FR-090-AC-1 | TC-110 | 🚧 Planned |
| FR-090 | FR-090-AC-2 | TC-110 | 🚧 Planned |
| FR-090 | FR-090-AC-3 | TC-113 | 🚧 Planned |
| FR-090 | FR-090-AC-4 | TC-113 | 🚧 Planned |
| FR-090 | FR-090-AC-5 | TC-117 | 🚧 Planned |
| FR-090 | FR-090-AC-7 | TC-238 | 🚧 Planned |
| FR-091 | FR-091-AC-1 | TC-111 | 🚧 Planned |
| FR-091 | FR-091-AC-2 | TC-112 | 🚧 Planned |
| FR-091 | FR-091-AC-3 | TC-111 | 🚧 Planned |
| FR-091 | FR-091-AC-4 | TC-116 | 🚧 Planned |
| FR-091 | FR-091-AC-5 | TC-115 | 🚧 Planned |
| FR-091 | FR-091-AC-6 | TC-118 | 🚧 Planned |
| FR-092 | FR-092-AC-1 | TC-112 | 🚧 Planned |
| FR-092 | FR-092-AC-2 | TC-112 | 🚧 Planned |
| FR-092 | FR-092-AC-3 | TC-112 | 🚧 Planned |
| FR-092 | FR-092-AC-4 | TC-118 | 🚧 Planned |
| FR-092 | FR-092-AC-5 | TC-117 | 🚧 Planned |
| FR-092 | FR-092-AC-6 | TC-112, TC-118 | 🚧 Planned |
| FR-092 | FR-092-AC-7 | TC-200 | 🚧 Planned |
| FR-093 | FR-093-AC-1 | TC-114 | 🚧 Planned |
| FR-093 | FR-093-AC-2 | TC-114 | 🚧 Planned |
| FR-093 | FR-093-AC-3 | TC-114 | 🚧 Planned |
| FR-093 | FR-093-AC-4 | TC-114 | 🚧 Planned |
| FR-093 | FR-093-AC-5 | TC-116 | 🚧 Planned |
| FR-094 | FR-094-AC-1 | TC-115 | 🚧 Planned |
| FR-094 | FR-094-AC-2 | TC-115 | 🚧 Planned |
| FR-094 | FR-094-AC-3 | TC-116 | 🚧 Planned |
| FR-094 | FR-094-AC-4 | TC-115 | 🚧 Planned |
| FR-094 | FR-094-AC-5 | TC-116 | 🚧 Planned |
| FR-094 | FR-094-AC-6 | TC-118 | 🚧 Planned |
| FR-094 | FR-094-AC-7 | TC-116 | 🚧 Planned |
| FR-094 | FR-094-AC-8 | TC-115 | 🚧 Planned |
| FR-094 | FR-094-AC-9 | TC-116 | 🚧 Planned |
| FR-094 | FR-094-AC-10 | TC-115 | 🚧 Planned |
| FR-295 | FR-295-AC-1, FR-295-AC-2, FR-295-AC-3, FR-295-AC-4 | TC-116 | 🚧 Planned |
| FR-289 | FR-289-AC-1, FR-289-AC-2, FR-289-AC-3, FR-289-AC-4, FR-289-AC-5 | TC-115 | 🚧 Planned |
| FR-293 | FR-293-AC-1, FR-293-AC-2, FR-293-AC-3 | TC-112, TC-118 | 🚧 Planned |
| FR-095 | FR-095-AC-1 | TC-117 | 🚧 Planned |
| FR-095 | FR-095-AC-2 | TC-117 | 🚧 Planned |
| FR-095 | FR-095-AC-3 | TC-117 | 🚧 Planned |
| FR-095 | FR-095-AC-4 | TC-117 | 🚧 Planned |
| FR-095 | FR-095-AC-5 | TC-117 | 🚧 Planned |
| FR-095 | FR-095-AC-6 | TC-113 | 🚧 Planned |
| FR-095 | FR-095-AC-7 | TC-117 | 🚧 Planned |
| FR-095 | FR-095-AC-8 | TC-117 | 🚧 Planned |
| FR-095 | FR-095-AC-9 | TC-200 | 🚧 Planned |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
| --- | --- | --- | --- |
| NFR-040 | property-based-testing, fault-injection, model-based-test-generation and integration-testing | TC-118 covers NFR-040-AC-1 through NFR-040-AC-4 | 🚧 Planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-110 | Select one exact temporal profile and clock | Integration | P0 | FR-090-AC-1, FR-090-AC-2 | 🚧 Planned |
| TC-111 | Evaluate bounded future truth tables | Property | P0 | FR-091-AC-1, FR-091-AC-3 | 🚧 Planned |
| TC-112 | Distinguish closure boundaries and bounded past history | Property | P0 | FR-091-AC-2, FR-092-AC-1, FR-092-AC-2, FR-092-AC-3, FR-092-AC-6, FR-293-AC-1, FR-293-AC-3 | 🚧 Planned |
| TC-113 | Distinguish event positions, samples and event time | Integration | P0 | FR-090-AC-3, FR-090-AC-4, FR-095-AC-6 | 🚧 Planned |
| TC-114 | Isolate temporal triggers and immutable captures | Integration | P0 | FR-093-AC-1, FR-093-AC-2, FR-093-AC-3, FR-093-AC-4 | 🚧 Planned |
| TC-115 | Settle inclusive deadlines from valid progress | Integration | P0 | FR-091-AC-5, FR-094-AC-1, FR-094-AC-2, FR-094-AC-4, FR-094-AC-8, FR-094-AC-10, FR-289-AC-1..AC-5 | 🚧 Planned |
| TC-116 | Separate open, closed and incomplete temporal inputs | Integration | P0 | FR-091-AC-4, FR-093-AC-5, FR-094-AC-3, FR-094-AC-5, FR-094-AC-7, FR-094-AC-9, FR-295-AC-1..AC-4 | 🚧 Planned |
| TC-117 | Prove or refuse native-to-TL temporal correspondence | Integration | P0 | FR-090-AC-5, FR-092-AC-5, FR-095-AC-1, FR-095-AC-2, FR-095-AC-3, FR-095-AC-4, FR-095-AC-5, FR-095-AC-7, FR-095-AC-8 | 🚧 Planned |
| TC-118 | Refuse temporal resource and retained-state exhaustion | Property | P0 | FR-091-AC-6, FR-092-AC-4, FR-092-AC-6, FR-094-AC-6, FR-293-AC-2, FR-293-AC-3, NFR-040-AC-1, NFR-040-AC-2, NFR-040-AC-3, NFR-040-AC-4 | 🚧 Planned |
| TC-200 | Infinite-trace provider boundary | Property | P0 | FR-095-AC-9 | 🚧 Planned |
| TC-238 | Unbounded temporal operator grammar admission | Property | P0 | FR-090-AC-7 | 🚧 Planned |

## Integration Test Matrix

| Integration ID | Purpose | Target Project | Type | Test Cases | Status |
| --- | --- | --- | --- | --- | --- |
| IT-040 | Supported native future correspondence | quire-contract-ir / tl-syntax / tl-mltl | library | TC-110, TC-111, TC-112, TC-113, TC-114, TC-117 | 🚧 Planned |
| IT-041 | Unsupported clock/past/boundary and lifecycle refusals | quire-contract-ir / tl-syntax / tl-mltl | library | TC-112, TC-115, TC-116, TC-117, TC-118 | 🚧 Planned |

## Evidence Limits

These artifacts define expected meaning and planned checks. They do not claim a
native temporal parser, bridge, TL past-time implementation, timestamped monitor
or executed differential campaign. No hosted workflow was added or dispatched.
