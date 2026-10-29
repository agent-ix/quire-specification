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
retain the selected domain package identity and version and population declaration
key, declared finite membership rule, population and snapshot/window keys, bounds,
observation sources, configuration/closure identities and versions, and
completeness/progress dependencies.

When selecting a v1 membership rule, the observation service SHALL admit only
`quire.observation.explicit-members/v1` with a sorted distinct bounded list of
every required semantic member identity; the rule is identified by that contract
and list, and no digest of the list is derived.

When retaining configuration and closure definitions, the observation
service SHALL identify each by its identity and version, and SHALL identify the
domain package by its identity and version and its declarations by (domain
package identity, IR node identity). No digest of definition bytes is retained.

When selecting a snapshot, the observation service SHALL identify it by the
exact declaration selection, membership rule, capture assertion and sorted
distinct admitted observation identities, componentwise.

When declaring a population, the observation service SHALL identify it by the
declaration selection, membership rule, snapshot-or-window selection,
observation sources, configuration and closure selections, and
completeness/progress dependencies, componentwise.

## Behavior

Unknown required membership, missing external-provider observations, or an absent declared closure input are incomplete/refused assessment causes and cannot establish aggregate success. The request SHALL retain the distinction between a selected snapshot and a selected event-time window.
When a declaration key outside the checked package, missing selected identity or version,
identity or version mismatch, or ambient default is encountered, the assessment service SHALL refuse it rather
than supplying a population, membership, configuration, or closure input.
When admitting a population, the assessment service SHALL take its closed extent
only from the population declaration in the checked package and SHALL NOT
derive it from observation records.
For a selected window `[start, end)`, the assessment service SHALL
include an exactly listed member at `start` and exclude one at `end`;
the latter can participate only in a separately selected later window.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-112-AC-1 | Changing a declared snapshot/window or membership rule changes the assessment subject. | Test (TC-142) |
| FR-112-AC-2 | Unknown membership and a missing provider record remain visible incomplete reasons. | Test (TC-142) |
| FR-112-AC-3 | A complete bounded population retains membership and closure provenance. | Test (TC-142) |
| FR-112-AC-4 | A declaration key, identity, version, membership-rule, configuration, or closure identity or version mismatch is refused without ambient defaulting. | Test (TC-142) |
| FR-112-AC-5 | A selected half-open window includes an exact member at `start`, excludes one at `end`, and cannot double-count either across windows. | Test (TC-142) |
| FR-112-AC-6 | The v1 explicit-members rule, snapshot and population keys compare equal exactly when every key component is equal; every key mutation makes the applicable key compare unequal and presentation permutation does not. | Test (TC-142) |
| FR-112-AC-7 | Unknown/duplicate/unbounded member sets refuse, while unknown required membership remains distinct from an empty required set. | Test (TC-142) |

## Dependencies

- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md).
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md): population declarations and their closed extents are spec-declared and keyed by domain package identity and IR node identity; E owns closure semantics.

## Status

Accepted by FS04 for bounded population, window, membership and completeness
meaning. Concrete assessment consumers remain independently tracked.
