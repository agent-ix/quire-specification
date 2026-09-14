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
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
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
| Forbidding selection | FR-057 Behavior, FR-057-AC-3 | `commit Node` makes that exact selected commit the forbidding boundary for this compensation; no independent disposition field exists. |
| Commit observation | FR-057 Inputs | The observed commit the boundary is checked against. |
| Owning registration | FR-056 Description | Each registration is bound to its commit boundary ([FR-233](FR-233-compensation-registration.md)). |

## Identity and comparison

- **Identity kind** — *derived*. The boundary is an authored commit control; its
  static identity is derived from its exact linked definition and semantic
  control handle, and its concrete boundary identity is the corresponding
  FR-221 node-occurrence key. A concrete commit observation is separately
  authored by the selected observation producer.
- **Key** — static boundary key (`qualified protocol-definition identity`,
  `declaration-local commit-control handle`); concrete boundary-occurrence key
  is FR-221; concrete observation key is one exact
  [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) of kind
  `commit`, bound to that occurrence and its commit requirement. The explicit
  `never` alternative is a value with no commit identity, not an empty boundary.
- **Key order** — lexicographic over the applicable static, occurrence or
  FR-287 key for canonical comparison only. The commit ordering checked by
  FR-057 is causal order and SHALL NOT be read from any key order.
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
- **Digest domain** — the linked definition and FR-287 authority digest retain
  their selected FR-201 domains.

`commit Node` selects exactly one forbidding static boundary for the owning
compensation definition. Only an established occurrence/observation of that
selected boundary forbids later registration, compensation attempts/effects or
recovery for its registrations; other commit nodes do not. `commit never` is a
present alternative selecting no earlier cutoff. It neither supplies a default
commit identity nor establishes perpetual/open-ended closure. When a selected
non-`never` boundary lacks required runtime evidence, the evaluation remains
pending/incomplete rather than treating the missing fact as permission.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-237-AC-1 | Registration after a forbidding commit and compensation after a forbidding commit are distinguishable violations. | Analysis |
| FR-237-AC-2 | A missing commit observation yields incomplete or pending and compares only to absence. | Analysis |
| FR-237-AC-3 | Commit ordering is read from the authored causal order and never from a key order, timestamp or source position. | Analysis |
| FR-237-AC-4 | A commit boundary, its concrete occurrence and its observation retain their separate declared keys; `never` has no commit identity and receives no default. | Analysis |
| FR-237-AC-5 | Reusing one commit-observation key for another boundary occurrence refuses, while sorting commit keys changes no authored causal order. | Analysis |
| FR-237-AC-6 | Only the exact non-`never` boundary selected by one compensation forbids its later registration, attempts, effects and recovery; another commit does not. | Test (TC-057) |
| FR-237-AC-7 | `commit never` selects no earlier cutoff and establishes no unbounded closure; missing evidence for a selected non-`never` boundary remains pending/incomplete rather than permitting by default. | Test (TC-057) |

## Dependencies

- [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md) establishes the
  commit-ordering check and the forbidding-commit violations.
- [FR-056](../../functional/protocol/FR-056-register-compensation.md) binds each
  registration to its commit boundary.
- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) supplies the
  authored causal order.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-221](FR-221-protocol-node-occurrence.md) defines the concrete boundary
  occurrence.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) defines the
  concrete commit-observation key.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) supplies the
  qualified linked-definition component.

## Status

Proposed closure for specification issue #31. Specification issue #28 fixed
boundary, occurrence and observation identity. Spec Language already publishes
the exact `commit Node`/`commit never` selection and has no independent
disposition field. Executable evaluator evidence remains planned under TC-057.
