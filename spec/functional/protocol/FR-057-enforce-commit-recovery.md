---
id: FR-057
title: "Enforce commit boundaries and authored recovery relations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-056
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-234
    type: references
  - target: ix://agent-ix/quire-specification/FR-237
    type: references
  - target: ix://agent-ix/quire-specification/FR-238
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
## Description

When evaluating compensation, the conformance evaluator SHALL check authored commit ordering and decide recovery only through the compensation's exact typed recovery relation.

## Inputs

A registered compensation obligation, commit observations, compensation
attempt/effect observations, immutable captures and an authored recovery
predicate evaluated under the selected state/temporal profiles.

## Outputs

A compensation obligation result with separate attempt, effect, commit and
recovery facts, or a typed violation/incomplete outcome.

## Behavior

The evaluator SHALL reject or violate registration after a forbidding commit,
compensation before the paired effect and compensation after a forbidding commit.
The evaluator SHALL NOT infer recovery success from compensation-operation success.
The evaluator SHALL retain the target and achieved states used by the recovery relation.

An authored commit boundary is either the explicit value `never` or a distinct
static subject keyed by the qualified protocol-definition identity and exact
declaration-local commit-control handle. Its concrete boundary occurrence uses
the FR-049 protocol-node-occurrence key. `never` has no commit identity and is
not an empty/default commit. A concrete commit observation is keyed by one
FR-035 authority-qualified opaque runtime reference of kind `commit` and SHALL
bind separately to the exact boundary occurrence and commit binding requirement.

An authored recovery relation is keyed by its FR-288 qualified protocol-
definition identity, exact declaration-local compensation handle and exact
declaration-local typed recovery-expression handle. The admitted typed
expression SHALL retain every selected model, state, temporal and clock semantic
dependency; a missing or mismatched dependency refuses linking rather than
being guessed into the key. This local handle does not claim the source-free
typed-expression identity that FR-015 says current native syntax lacks. Target
and achieved state values are operands, not key components. Changing an operand
value changes the evaluation, not which authored relation was selected.

For each compensation definition, `commit Node` selects the one exact static
commit-control boundary that forbids later registration, compensation attempts,
compensation effects and recovery for its registrations. Other commit controls
do not forbid that compensation. `commit never` selects no earlier cutoff; it
does not assert that the obligation or surrounding execution is closed forever.
An established observation of the selected commit produces distinct
after-commit violations. When a selected non-`never` boundary's required
runtime commit evidence is unavailable, the check remains pending/incomplete
and never grants permission by default.

These tuples have only the mechanical lexicographic key order of FR-200. Commit
ordering remains the authored causal order and SHALL NOT be inferred from a key,
timestamp, source position or observation arrival. Which commits have a
forbidding disposition remains governed separately and is not decided by these
identity rules.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-057-AC-1 | A full refund effect before commit satisfies a full-restoration relation only when the captured amount and target state match. | Test (TC-057) |
| FR-057-AC-2 | A successful partial refund can satisfy an authored partial-recovery relation while failing full restoration. | Test (TC-057) |
| FR-057-AC-3 | Registration after the compensation's selected forbidding commit, compensation before the forward effect and compensation after that selected commit are distinct ordering violations; another commit does not trigger them. | Test (TC-057) |
| FR-057-AC-4 | Missing state, commit or compensation-effect inputs yield incomplete/pending rather than recovery success. | Test (TC-057) |
| FR-057-AC-5 | A commit boundary is identified by its qualified definition and exact commit-control handle, while `never` carries no commit identity and cannot compare equal to an empty or default boundary. | Test (TC-057) |
| FR-057-AC-6 | A commit observation without one exact authority-qualified `commit` identity and its matching boundary occurrence and binding requirement refuses; reusing that key for another boundary is an identity contradiction, and sorting valid keys changes no authored commit order. | Test (TC-057) |
| FR-057-AC-7 | Two recovery relations compare equal only when their qualified protocol-definition, compensation handle and typed recovery-expression handle all compare equal; the admitted expression retains its selected semantic dependencies, while target and achieved operand values do not identify the relation. | Test (TC-057) |
| FR-057-AC-8 | `commit Node` selects exactly one forbidding boundary for that compensation, while `commit never` selects no earlier cutoff and establishes no unbounded closure; a missing selected commit observation remains pending/incomplete. | Test (TC-057) |

## Dependencies

- [FR-056](FR-056-register-compensation.md).
- Agent A's state predicates and E's temporal/closure semantics.
- [FR-287](../../objects/protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  defines the concrete commit-observation key.
- [FR-288](../../objects/protocol/FR-288-qualified-protocol-definition-identity.md)
  supplies the qualified linked-definition component.
