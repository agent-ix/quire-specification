---
id: TM-007
title: "Observation and monitoring acceptance"
type: TestMatrix
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: references
  - target: ix://agent-ix/quire-specification/FR-111
    type: references
  - target: ix://agent-ix/quire-specification/FR-112
    type: references
  - target: ix://agent-ix/quire-specification/FR-113
    type: references
  - target: ix://agent-ix/quire-specification/FR-114
    type: references
  - target: ix://agent-ix/quire-specification/FR-115
    type: references
  - target: ix://agent-ix/quire-specification/FR-116
    type: references
  - target: ix://agent-ix/quire-specification/FR-285
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
  - target: ix://agent-ix/quire-specification/FR-289
    type: references
  - target: ix://agent-ix/quire-specification/FR-263
    type: references
  - target: ix://agent-ix/quire-specification/FR-260
    type: references
  - target: ix://agent-ix/quire-specification/FR-261
    type: references
  - target: ix://agent-ix/quire-specification/FR-264
    type: references
  - target: ix://agent-ix/quire-specification/FR-265
    type: references
  - target: ix://agent-ix/quire-specification/FR-266
    type: references
  - target: ix://agent-ix/quire-specification/FR-267
    type: references
  - target: ix://agent-ix/quire-specification/FR-268
    type: references
  - target: ix://agent-ix/quire-specification/FR-294
    type: references
---
## Overview

Draft OB01–03 and consumer-boundary acceptance only. Every case is planned and
unimplemented; it does not claim qualified replay, production monitoring, codegen
CLI, or backend conformance.

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
| --- | --- | --- | --- |
| FR-110 | FR-110-AC-1, FR-110-AC-2, FR-110-AC-3, FR-110-AC-4, FR-110-AC-5, FR-110-AC-6 | TC-140 | 🚧 Planned |
| FR-111 | FR-111-AC-1, FR-111-AC-2, FR-111-AC-3, FR-111-AC-4 | TC-141 | 🚧 Planned |
| FR-112 | FR-112-AC-1, FR-112-AC-2, FR-112-AC-3, FR-112-AC-4, FR-112-AC-5, FR-112-AC-6, FR-112-AC-7 | TC-142 | 🚧 Planned |
| FR-113 | FR-113-AC-1, FR-113-AC-2, FR-113-AC-3, FR-113-AC-4, FR-113-AC-5, FR-113-AC-6 | TC-143 | 🚧 Planned |
| FR-114 | FR-114-AC-1, FR-114-AC-2, FR-114-AC-3 | TC-144 | 🚧 Planned |
| FR-115 | FR-115-AC-1, FR-115-AC-2, FR-115-AC-3, FR-115-AC-4, FR-115-AC-5, FR-115-AC-6, FR-115-AC-7 | TC-145 | 🚧 Planned |
| FR-116 | FR-116-AC-1, FR-116-AC-2, FR-116-AC-3, FR-116-AC-4, FR-116-AC-5 | TC-146 | 🚧 Planned |
| FR-285 | FR-285-AC-1, FR-285-AC-2, FR-285-AC-3, FR-285-AC-4 | TC-145 | 🚧 Planned |
| FR-287 | FR-287-AC-1, FR-287-AC-2, FR-287-AC-3, FR-287-AC-4 | TC-145, TC-116 | 🚧 Planned |
| FR-288 | FR-288-AC-1, FR-288-AC-2, FR-288-AC-3, FR-288-AC-4, FR-288-AC-5 | TC-140, TC-143 | 🚧 Planned |
| FR-289 | FR-289-AC-1, FR-289-AC-2, FR-289-AC-3, FR-289-AC-4, FR-289-AC-5 | TC-143, TC-115 | 🚧 Planned |
| FR-260 | FR-260-AC-1, FR-260-AC-2, FR-260-AC-3, FR-260-AC-4 | TC-140 | 🚧 Planned |
| FR-261 | FR-261-AC-1, FR-261-AC-2, FR-261-AC-3, FR-261-AC-4 | TC-140 | 🚧 Planned |
| FR-263 | FR-263-AC-1, FR-263-AC-2, FR-263-AC-3, FR-263-AC-4, FR-263-AC-5 | TC-142 | 🚧 Planned |
| FR-264 | FR-264-AC-1, FR-264-AC-2, FR-264-AC-3, FR-264-AC-4, FR-264-AC-5 | TC-142 | 🚧 Planned |
| FR-265 | FR-265-AC-1, FR-265-AC-2, FR-265-AC-3, FR-265-AC-4, FR-265-AC-5 | TC-142 | 🚧 Planned |
| FR-266 | FR-266-AC-1, FR-266-AC-2, FR-266-AC-3, FR-266-AC-4 | TC-142 | 🚧 Planned |
| FR-267 | FR-267-AC-1, FR-267-AC-2, FR-267-AC-3, FR-267-AC-4, FR-267-AC-5 | TC-143 | 🚧 Planned |
| FR-268 | FR-268-AC-1, FR-268-AC-2, FR-268-AC-3, FR-268-AC-4, FR-268-AC-5, FR-268-AC-6, FR-268-AC-7 | TC-143 | 🚧 Planned |
| FR-294 | FR-294-AC-1, FR-294-AC-2, FR-294-AC-3 | TC-143 | 🚧 Planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-140 | Bind typed observations without implicit values | Integration | P1 | FR-110, FR-260, FR-261, FR-288 | 🚧 Planned |
| TC-141 | Correlate related instances without heuristics | Integration | P1 | FR-111 | 🚧 Planned |
| TC-142 | Declare population and completeness | Integration | P1 | FR-112, FR-263, FR-264, FR-265 | 🚧 Planned |
| TC-143 | Preserve time, progress, and late-data authority | Integration | P1 | FR-113, FR-267, FR-268, FR-288, FR-289, FR-294 | 🚧 Planned |
| TC-144 | Agree replay and incremental assessment | Integration | P1 | FR-114 | 🚧 Planned |
| TC-145 | Report observation adequacy without promotion | Integration | P1 | FR-115-AC-1, FR-115-AC-2, FR-115-AC-3, FR-115-AC-4, FR-115-AC-5, FR-115-AC-6, FR-115-AC-7, FR-285-AC-1, FR-285-AC-2, FR-285-AC-3, FR-285-AC-4 | 🚧 Planned |
| TC-146 | Map observation results to consumers | Integration | P1 | FR-116 | 🚧 Planned |
| TC-147 | Audit the observation executable boundary | Static | P1 | NFR-052 | 🚧 Planned |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
| --- | --- | --- | --- |
| NFR-050 | Test | TC-142, TC-144 | 🚧 Planned |
| NFR-051 | Test | TC-144, TC-145, TC-146 | 🚧 Planned |
| NFR-052 | Inspection | TC-147 | 🚧 Planned |

## Integration Test Matrix

| Purpose | Target | Type | Test Cases |
| --- | --- | --- | --- |
| Bind admitted observations, captures, progress, closure, and protocol obligations without reconstructing an identity. | E-owned temporal meaning and B-owned protocol assessment through their selected structured interfaces. | event | IT-050 |
| Deliver structured observation results and explicit mapping losses without promoting non-success, target-text parsing, or unqualified coverage evidence. | B-owned result contract, codegen/output consumer boundary, and selected export profile. | event | IT-051 |
