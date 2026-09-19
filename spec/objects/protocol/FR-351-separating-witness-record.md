---
id: FR-351
title: "Separating witness record"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-243
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-041
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-351: Separating witness record

## Description

When a truth result settles on a `decisive-witness` or `decisive-counterexample` basis, the result producer SHALL retain exactly one separating witness record with the deciding element, index, value path and trace position below, and a strict reader SHALL accept a witness only as this exact record or as its absence.

## Properties

| Property | Description |
| --- | --- |
| deciding element | The exact value that decided the result: the witness or counterexample value at the point evaluation stopped (FR-041-AC-4; FR-061; FR-094; FR-243). |
| index | The exact position of the deciding element within its enclosing bounded collection, where the family assigns a positional index (collection `forall`/`exists`, FR-041-AC-4). Absent for a family that assigns no positional index. |
| value path | The exact path from the assessed subject's root to the deciding element's location, including into a nested record, collection or model reference. |
| trace position | The exact position within the retained execution or observation trace at which the deciding element was produced or observed, where the family retains a trace (temporal, FR-094; protocol, FR-243). Absent for a family that retains no trace. |

## Family neutrality

This one record shape backs every family's decisive settlement, and no family defines a distinct shape:

- **Collection** — `forall`/`exists` stop at their specified decisive occurrence and retain its source binding (FR-041-AC-4); that occurrence and binding are the deciding element, index and value path above.
- **Temporal** — a decisive temporal witness or counterexample preserves the truth under every admitted continuation only with completed assessment execution, exact complete decision support and this record (FR-094, FR-061-AC-8).
- **Protocol** — an open decision scope settles `satisfied` or `violated` because the selected profile proves the retained witness or counterexample preserves that truth under every admitted continuation (FR-243 `decisive-witness`, `decisive-counterexample`).

## Witness-free dispositions

A separating witness accompanies only a `decisive-witness` or `decisive-counterexample` settlement basis (FR-243). Resource exhaustion or cancellation retains the incomplete-execution disposition, originating stage, charged work item and effective limit or cancellation source, and carries no witness. A valid checked subject with an unsupported requested backend capability retains the checked subject and emits no substitute witness in the deciding element's place. A strict reader accepts a witness only as the exact record above or as its absence; extending FR-094-AC-7's one-axis-substitution refusal to the witness axis, it refuses a record missing a required component, carrying a foreign shape, or substituting a default for absence on any one component.

## Identity and comparison

- **Identity kind** — *value*. A separating witness record has no identity of its
  own; it is compared componentwise over its key.
- **Key** — the deciding element identity, the index where the family assigns
  one, the value path, and the trace position where the family retains one.
  Equality is componentwise over the key and nothing else. Display name, source
  path, span, message text, locale and rendering never participate.
- **Key order** — **none is established.** No requirement states a total order
  over separating witness records, and this artifact supplies none.
- **Causal order** — none of its own. The decisive settlement it substantiates
  carries its own supersession lineage under FR-061 and FR-243, which is never
  derived from this record's key order, timestamp or source position.
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value. The record itself is either present in full or
  absent; there is no partial record. A witness absent under a `decisive-witness`
  or `decisive-counterexample` basis is a refusal, not an empty substitute
  (see **Witness-free dispositions**).
- **Digest domain** — where a key component is a digest, its domain is
  [`ix://agent-ix/quire-specification/FR-201`](../foundation/FR-201-canonical-identity-domain.md).
  An identity crossing domains is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-351-AC-1 | Every retained separating witness carries its exact deciding-element identity, plus an index where the family assigns one, a value path from the assessed subject's root, and a trace position where the family retains one. | Test (TC-254) |
| FR-351-AC-2 | The identical record shape backs a decisive bounded collection occurrence (FR-041), a decisive temporal witness or counterexample (FR-094, FR-061), and a decisive protocol witness or counterexample (FR-243, FR-061); no family defines a distinct witness shape. | Test (TC-254) |
| FR-351-AC-3 | A result with an exhausted or cancelled assessment execution carries no witness. | Test (TC-254) |
| FR-351-AC-4 | A result settling with an unsupported requested backend capability retains the checked subject and emits no substitute witness. | Test (TC-254) |
| FR-351-AC-5 | A strict reader accepts a witness only as the exact record above or as its absence; a record missing a required component, carrying a foreign shape, or substituting a default for absence on any one component is refused. | Test (TC-254) |
| FR-351-AC-6 | No ordering over separating witness records is asserted by this artifact or inferred from presentation text. | Inspection |

## Dependencies

- [FR-041](../../functional/foundation/FR-041-evaluate-ordered-queries.md) supplies
  the collection family's decisive occurrence and source binding.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) supplies
  the retained-witness/counterexample rule this record substantiates.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  supplies the temporal family's decisive-settlement conditions and the
  one-axis-substitution refusal this record extends.
- [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  supplies the replay envelope this record's shape now types.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain vocabulary.
- [FR-243](FR-243-settlement-basis.md) supplies the protocol family's
  `decisive-witness` and `decisive-counterexample` bases this record
  substantiates.
- [AD-003](../../assurance/AD-003-native-quire-language-charter.md) supplies the
  incomplete-execution and unsupported-projection dispositions this record
  stays witness-free under.

## Status

Draft. Gives the separating witness or counterexample that FR-061, FR-094, FR-243
and FR-041 already retain abstractly its exact field-level shape. Implementation
and replay evidence remain planned under
[agent-ix/quire-spec-language#186](https://github.com/agent-ix/quire-spec-language/issues/186),
[agent-ix/quire-contract-codegen#50](https://github.com/agent-ix/quire-contract-codegen/issues/50)
and [agent-ix/quire-verification#38](https://github.com/agent-ix/quire-verification/issues/38).
