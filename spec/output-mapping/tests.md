---
id: TM-008
title: "Output mapping and loss-report acceptance"
type: TestMatrix
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: references
  - target: ix://agent-ix/quire-specification/FR-121
    type: references
  - target: ix://agent-ix/quire-specification/FR-122
    type: references
  - target: ix://agent-ix/quire-specification/FR-123
    type: references
  - target: ix://agent-ix/quire-specification/FR-124
    type: references
  - target: ix://agent-ix/quire-specification/FR-125
    type: references
---
## Overview

FS06-accepted portable mapping controls, implemented by qcir #55/#56/#57.

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
| --- | --- | --- | --- |
| FR-120 | FR-120-AC-1..AC-4 | TC-150, TC-154 |  |
| FR-121 | FR-121-AC-1..AC-5 | TC-150, TC-154, TC-155 |  |
| FR-122 | FR-122-AC-1..AC-5 | TC-151, TC-155 |  |
| FR-123 | FR-123-AC-1..AC-5 | TC-152, TC-155 |  |
| FR-124 | FR-124-AC-1..AC-5 | TC-153, TC-155 |  |
| FR-125 | FR-125-AC-1..AC-5 | TC-150, TC-154, TC-155 |  |
| FR-297 | FR-297-AC-1..AC-4 | TC-150, TC-154, TC-155 |  |
| FR-298 | FR-298-AC-1..AC-5 | TC-150, TC-154 |  |
| FR-299 | FR-299-AC-1..AC-5 | TC-150, TC-154, TC-155 |  |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-150 | Account for every output obligation | Integration | P0 | FR-120, FR-121, FR-297, FR-298, FR-299 | 🚧 |
| TC-151 | Generate bounded OCL 2.4 output | Integration | P0 | FR-122 | 🚧 |
| TC-152 | Generate bounded SysML/KerML output | Integration | P1 | FR-123 | 🚧 |
| TC-153 | Generate bounded FRETish output | Integration | P1 | FR-124 | 🚧 |
| TC-154 | Refuse mapping mismatch and exhaustion | Property | P0 | FR-120, FR-121, FR-125, NFR-060 | 🚧 |
| TC-155 | Separate structural observation from preservation | Integration | P0 | FR-122..FR-125, NFR-061 | 🚧 |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
| --- | --- | --- | --- |
| NFR-060 | property, boundary, fault-injection, metamorphic | TC-154, TC-155 |  |
| NFR-061 | Cargo.toml review, adverse test | TC-155 |  |

## Integration Test Matrix

| Purpose | Target | Type | Test Cases |
| --- | --- | --- | --- |
| Prove three independent exporters share strict intake, loss, identity, resource and package behavior. | qcir #55/#56/#57 | file/API | IT-060 |
