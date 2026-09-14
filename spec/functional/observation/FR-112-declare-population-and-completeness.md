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

When selecting a v1 membership rule, the observation service SHALL admit only
`quire.observation.explicit-members/v1` with a sorted distinct bounded list of
every required semantic member identity and derive its identity under
`quire.verification.jcs` from that exact versioned list.

When retaining producer-interface, configuration and closure definition bytes,
the observation service SHALL identify their digests under FR-201
`raw-artifact-digest`.

When selecting a snapshot, the observation service SHALL derive its
`sha256-jcs` identity from the exact producer selection, membership-rule
identity, capture assertion and sorted distinct admitted observation identities
under `quire.observation.snapshot-identity/v1-draft.1`.

When declaring a population, the observation service SHALL derive its
`sha256-jcs` identity from the producer selection, membership rule,
snapshot-or-window selection, observation sources, configuration and closure
selections, and completeness/progress dependencies under
`quire.observation.population-identity/v1-draft.1`.

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
| FR-112-AC-6 | The v1 explicit-members rule, snapshot and population identities are independently recomputable; every key mutation changes the applicable identity and presentation permutation does not. | Test (TC-142) |
| FR-112-AC-7 | Unknown/duplicate/unbounded member sets and absent/foreign digest domains refuse, while unknown required membership remains distinct from an empty required set. | Test (TC-142) |

## Dependencies

- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md).
- D owns the selected Producer interface 1.2.0 population/model/configuration identity; E owns closure semantics.

## Status

Accepted by FS04 for bounded population, window, membership and completeness
meaning. Concrete assessment consumers remain independently tracked.
