---
id: FR-236
title: "Partition compensation attempt and aggregate recovery outcomes"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-058
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-235
    type: references
  - target: ix://agent-ix/quire-specification/FR-238
    type: references
  - target: ix://agent-ix/quire-specification/FR-260
    type: references
---
# FR-236: Partition Compensation Attempt and Aggregate Recovery Outcomes

## Description

When multiple compensation observations relate to one obligation, the
conformance evaluator SHALL preserve a per-attempt outcome separately from
aggregate recovery facts. The accepted five-member vocabulary is partitioned
by subject; a consumer SHALL NOT approximate the partition from a label,
arrival order, effect identity, or recovery result (FR-058).

## Values

| Value | Description |
| --- | --- |
| `failed` | Admitted for a per-attempt outcome and for a recovery-relation aggregate fact. The attempt did not succeed; the aggregate relation evaluated false with its required inputs. |
| timed_out | Wire spelling `timed-out`. Admitted for a per-attempt outcome and for a recovery-relation aggregate fact. The attempt timed out; the aggregate relation reached its declared timeout without a recovery result. |
| `pending` | Admitted for a per-attempt outcome and for a recovery-relation aggregate fact. It is explicit incomplete evidence, never an absence encoding, and cannot establish recovery success. |
| `successful` | Admitted for a per-attempt outcome and for a recovery-relation aggregate fact. A successful attempt alone does not establish the aggregate fact. |
| `duplicate` | Forbidden for a per-attempt outcome and admitted only for a duplicate-business-effect aggregate fact. It is not an attempt outcome, retry identity, or recovery-relation result. |

## Admitted subjects

An aggregate recovery fact is either a recovery-relation fact carrying one of
`failed`, `timed-out`, `pending`, or `successful`, or a duplicate-business-effect
fact carrying `duplicate`. A duplicate-business-effect fact may coexist with a
per-attempt outcome and a recovery-relation fact because they describe distinct
identified subjects; neither fact overwrites, relabels, or ranks the other.

## Identity, absence and comparison

- **Per-attempt outcome identity** — a value fact keyed by the exact
  `compensation-attempt` FR-287 identity and the exact admitted-observation
  FR-260 identity that supplies the outcome. The label is the fact's value, not
  an identity component. For one selected attempt/observation pair, exactly one
  admitted label may be retained; two different labels for that pair are an
  identity contradiction and the evaluator SHALL refuse them rather than retain
  two facts. Two labels compare unequal, and an attempt identity never compares
  equal to an effect or aggregate-fact identity.
- **Recovery-relation aggregate identity** — a value fact keyed by the exact
  FR-233 registration, FR-238 recovery relation, and the declared population
  scope, membership, and completeness authorities that the relation evaluates.
  Its label is compared exactly; its key order is canonical comparison only and
  supplies no recovery precedence.
- **Duplicate-business-effect aggregate identity** — a value fact keyed by the
  exact FR-233 registration and the exact affected `business-effect` FR-287
  identity, including its declared duplicate relation. It is separate from a
  compensation-attempt key even where an attempt causally established the
  effect.
- **Absence** — no reported fact is absent, and absence compares only to
  absence. Absence is never defaulted to `pending`, `failed`, `timed-out`,
  `duplicate`, or `successful`. `pending` is an explicit admitted label with
  retained incomplete evidence, not an absence encoding.
- **Comparison** — labels use exact lower-case hyphenated strings. No spelling
  normalization, severity ordering, timestamp ordering, or fallback subject
  conversion is permitted.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-236-AC-1 | A per-attempt outcome admits exactly `failed`, `timed-out`, `pending`, or `successful` and refuses `duplicate`. | Property Test (TC-058) |
| FR-236-AC-2 | A recovery-relation aggregate fact admits exactly `failed`, `timed-out`, `pending`, or `successful`, while a duplicate-business-effect aggregate fact admits exactly `duplicate`; neither substitutes for the other. | Property Test (TC-058) |
| FR-236-AC-3 | A `duplicate` business effect is keyed as an aggregate effect fact, remains distinct from each compensation attempt and its outcome, and may coexist with them without relabelling either. | Property Test (TC-058) |
| FR-236-AC-4 | Successful attempts alone do not establish successful recovery, and failed or timed-out earlier attempts remain retained after a later successful retry. | Property Test (TC-057, TC-058) |
| FR-236-AC-5 | Independent mutations of subject kind, admitted label, identity component, required aggregate authority, and absence are refused or remain distinct exactly as the partition requires; two different labels for one exact attempt/observation pair refuse as an identity contradiction. | Property Test (TC-058) |

## Dependencies

- [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md) establishes
  that recovery success is not inferred from compensation-operation success.
- [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md)
  establishes retry retention, aggregate recovery, and duplicate-effect distinctions.
- [FR-235](FR-235-compensation-attempt.md) identifies the compensation attempt.
- [FR-238](FR-238-recovery-relation.md) identifies the authored recovery relation.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) supplies exact
  runtime subject identities.
- [FR-260](../observation/FR-260-admitted-observation.md) supplies the exact admitted
  observation identity for a per-attempt outcome fact.

## Status

Accepted semantic ruling for specification issue #57. This specification-only
partition authorizes no Protocol implementation, native-v1 resource, tag, or
downstream consumer change.
