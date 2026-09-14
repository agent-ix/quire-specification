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
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-292: Analysis backend selection

## Description

When a selected backend executes a protocol claim, the dispatcher SHALL retain
the exact backend interface identity, revision and implementation digest plus
the admitted fragment and claim capability used for that execution.

## Properties

- **interface identity** — nonempty owner-qualified backend contract identity.
- **revision** — positive immutable backend-interface revision.
- **implementation digest** — exact implementation/package digest and selected FR-201 domain.
- **admitted fragment** — exact FR-291 key.
- **claim capabilities** — sorted distinct nonempty FR-290 member set.

## Identity and comparison

- **Identity kind** — *authored* by the backend owner.
- **Key** — (interface identity, revision, digest domain, implementation digest).
- **Key order** — lexicographic for deterministic collections only; it is not a
  preference, fallback or recency order.
- **Causal order** — none unless an explicit successor relation is supplied.
- **Absence** — a result for an executed advanced claim without a backend is
  refused; an unsupported claim retains the backend selection and unsupported
  disposition when the dispatcher reached that boundary.
- **Digest domain** — the selected FR-201 domain is part of the key.

The intersection of the selected backend capability and selected admitted
fragment determines whether dispatch is supported. No backend may enlarge the
fragment, inherit another backend's success, or invoke an unreviewed fallback.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-292-AC-1 | Mutating interface identity, revision, digest domain, implementation digest, fragment or claim capability prevents result reuse. | Test (TC-060, TC-061) |
| FR-292-AC-2 | An unsupported claim and a failed supported execution remain distinct and retain the exact backend selection. | Test (TC-060) |
| FR-292-AC-3 | A failed backend invokes no fallback and cannot change an independent result. | Test (TC-060) |

## Dependencies

- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)
  declares backend selection, failure and no-fallback behavior.
- [FR-290](FR-290-protocol-claim-kind.md) defines claim capabilities.
- [FR-291](FR-291-admitted-analysis-fragment.md) defines the fragment the
  backend cannot enlarge.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) defines digest
  domain comparison.

## Status

Proposed shared ruling for `quire-specification#40`.
