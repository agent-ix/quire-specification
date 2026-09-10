---
id: FR-112
title: "Declare finite population and window completeness"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
---
## Description

When assessing a population or event-time window, the assessment request SHALL retain its declared finite membership rule, snapshot/window identity, bounds, observation sources, and completeness/progress dependencies.

## Behavior

Unknown required membership, missing external-provider observations, or an absent declared closure input SHALL yield incomplete or refused assessment and SHALL NOT establish aggregate success. The request SHALL retain the distinction between a selected snapshot and a selected event-time window.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-112-AC-1 | Changing a declared snapshot/window or membership rule changes the assessment subject. | Test (TC-142) |
| FR-112-AC-2 | Unknown membership and a missing provider record remain visible incomplete reasons. | Test (TC-142) |
| FR-112-AC-3 | A complete bounded population retains membership and closure provenance. | Test (TC-142) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- D owns population/model identity; E owns closure semantics.
