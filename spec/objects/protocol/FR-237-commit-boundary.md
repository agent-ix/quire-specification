---
id: FR-237
title: "Commit boundary"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-057
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-056
    type: specifies
---
# FR-237: Commit boundary

## Description

When evaluating compensation, the conformance evaluator SHALL check authored commit ordering (FR-057).

FR-057 adds that the evaluator "SHALL reject or violate registration after a
forbidding commit, compensation before the paired effect and compensation after a
forbidding commit."

FR-056 requires each registration to be bound to its commit boundary, and FR-057
Outputs requires "separate attempt, effect, commit and recovery facts".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Authored commit ordering | FR-057 Description | The evaluator checks it; it is authored, not inferred. |
| Forbidding disposition | FR-057 Behavior, FR-057-AC-3 | Whether the commit forbids a later registration or compensation. |
| Commit observation | FR-057 Inputs | The observed commit the boundary is checked against. |
| Owning registration | FR-056 Description | Each registration is bound to its commit boundary ([FR-233](FR-233-compensation-registration.md)). |

## Identity and comparison

- **Identity kind** — *authored* in ordering only. FR-057 states that the
  evaluator checks "authored commit ordering", so the boundary and its position
  are written rather than inferred. No requirement states what identifies one
  commit boundary, so the identity kind for comparison purposes is not fully
  established; see the open gap.
- **Key** — not established. No requirement names the components that identify a
  commit boundary or a commit observation. The prose states only what the key is
  not: under FR-009 and FR-047, display name, source path, span, message text,
  locale and rendering never participate, and under FR-051 a transport trace
  identifier or equal payload alone identifies nothing.
- **Key order** — none. The key components are not fixed by the prose, so no
  total order over them can be declared. In particular the commit ordering
  checked by FR-057 is a causal order, not a key order, and must not be read as
  one.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). FR-057-AC-3
  names the violations this order detects: "Registration after commit and
  compensation before the forward effect or after a forbidding commit are
  distinct ordering violations." The order is authored and is never derived from
  key order, timestamp or source position (FR-052-AC-2).
- **Absence** — an absent commit observation compares only to absence.
  FR-057-AC-4 states that "Missing state, commit or compensation-effect inputs
  yield incomplete/pending rather than recovery success"; an absent commit is
  never compared to a default, an empty commit or an implicit permitting commit.
- **Digest domain** — no key component is established, so none is a digest here.
  Where a commit identity is carried as a digest, it retains the domain selected
  under [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-237-AC-1 | Registration after a forbidding commit and compensation after a forbidding commit are distinguishable violations. | Analysis |
| FR-237-AC-2 | A missing commit observation yields incomplete or pending and compares only to absence. | Analysis |
| FR-237-AC-3 | Commit ordering is read from the authored causal order and never from a key order, timestamp or source position. | Analysis |
| FR-237-AC-4 | No key order is declared for this object. | Inspection |

## Dependencies

- [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md) establishes the
  commit-ordering check and the forbidding-commit violations.
- [FR-056](../../functional/protocol/FR-056-register-compensation.md) binds each
  registration to its commit boundary.
- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) supplies the
  authored causal order.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

The prose establishes that a commit boundary exists, is authored, is bound to a
registration, and may forbid a later registration or compensation. It does not
establish:

- **Identity** — whether a commit boundary is a distinct identified subject or
  only a position in the authored order.
- **Key** — which components identify one commit boundary or one commit
  observation.
- **Order** — no key order follows, since the components are unfixed.

FR-057 also uses two spellings for the same check: FR-057 Behavior reads
"registration after a forbidding commit", while FR-057-AC-3 reads "Registration
after commit". Whether every commit forbids registration, or only a commit with a
forbidding disposition, is not stated. No key, order or disposition rule is
invented here.

## Status

Draft. This artifact records existing meaning structurally. It establishes no new
obligation and claims no implementation coverage.
