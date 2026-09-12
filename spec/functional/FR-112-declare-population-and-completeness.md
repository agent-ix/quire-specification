---
id: FR-112
title: "Declare finite population and window completeness"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
  - target: ix://agent-ix/quire-specification/FR-263
    type: references
  - target: ix://agent-ix/quire-specification/FR-264
    type: references
  - target: ix://agent-ix/quire-specification/FR-265
    type: references
  - target: ix://agent-ix/quire-specification/FR-266
    type: references
---
## Description

When assessing a population or event-time window, the assessment request SHALL
retain the selected Producer interface 1.2.0 identity/version/digest, declared
finite membership rule/digest, population and snapshot/window identities, bounds,
observation sources, configuration/closure identities and digests, and
completeness/progress dependencies.

## Behavior

Unknown required membership, missing external-provider observations, or an absent declared closure input are incomplete/refused assessment causes and cannot establish aggregate success. The request SHALL retain the distinction between a selected snapshot and a selected event-time window.
When an unknown producer version, missing selected identity, digest mismatch, or
ambient default is encountered, the assessment service SHALL refuse it rather
than supplying a population, membership, configuration, or closure input.
For a selected FCD 1.2.0 window `[start, end)`, the assessment service SHALL
include an exactly listed/digested member at `start` and exclude one at `end`;
the latter can participate only in a separately selected later window.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-112-AC-1 | Changing a declared snapshot/window or membership rule changes the assessment subject. | Test (TC-142) |
| FR-112-AC-2 | Unknown membership and a missing provider record remain visible incomplete reasons. | Test (TC-142) |
| FR-112-AC-3 | A complete bounded population retains membership and closure provenance. | Test (TC-142) |
| FR-112-AC-4 | A producer version, identity, membership-digest, configuration, or closure-digest mismatch is refused without ambient defaulting. | Test (TC-142) |
| FR-112-AC-5 | A selected half-open window includes an exact member at `start`, excludes one at `end`, and cannot double-count either across windows. | Test (TC-142) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- D owns the selected Producer interface 1.2.0 population/model/configuration identity; E owns closure semantics.
