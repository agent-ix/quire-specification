---
id: FR-292
title: "Analysis backend selection"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-060
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-291
    type: references
---
# FR-292: Analysis backend selection

## Description

When a selected backend executes a protocol claim, the dispatcher SHALL retain
the exact backend interface identity plus the admitted fragment and claim capability used for that execution.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| interface_identity | String | 1..1 |  |
| admitted_fragment | String | 1..1 |  |
| claim_capabilities | String | 1..1 |  |

## Field sources

- **interface identity** — nonempty owner-qualified backend contract identity.
- **admitted fragment** — exact FR-291 key.
- **claim capabilities** — sorted distinct nonempty set of FR-290 protocol-family members; the interface selects the FR-290 vocabulary `quire.capability-kind/v1`.

## Identity and comparison

- **Identity kind** — *authored* by the backend owner.
- **Key** — interface identity.
- **Key order** — lexicographic for deterministic collections only; it is not a
  preference, fallback or recency order.
- **Causal order** — none unless an explicit successor relation is supplied.
- **Absence** — a result for an executed advanced claim without a backend is
  refused; an unsupported claim retains the backend selection and unsupported
  disposition when the dispatcher reached that boundary.

The intersection of the selected backend capability and selected admitted
fragment determines whether dispatch is supported. No backend may enlarge the
fragment, inherit another backend's success, or invoke an unreviewed fallback.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-292-AC-1 | Mutating interface identity, fragment or claim capability prevents result reuse. | Test (TC-060, TC-061) |
| FR-292-AC-2 | An unsupported claim and a failed supported execution remain distinct and retain the exact backend selection. | Test (TC-060) |
| FR-292-AC-3 | A failed backend invokes no fallback and cannot change an independent result. | Test (TC-060) |

## Dependencies

- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)
  declares backend selection, failure and no-fallback behavior.
- [FR-290](FR-290-protocol-claim-kind.md) defines claim capabilities.
- [FR-291](FR-291-admitted-analysis-fragment.md) defines the fragment the
  backend cannot enlarge.

## Status

Proposed shared ruling for `quire-specification#40`.
