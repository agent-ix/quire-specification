---
id: FR-352
title: "Produce and read the native-run-result/2 wire"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-014
    type: references
  - target: ix://agent-ix/quire-specification/FR-351
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-207
    type: references
---
# FR-352: Produce and read the native-run-result/2 wire

## Description

When a native run result is produced or read, the producer and the strict
reader SHALL use the closed `native-run-result/2` clause-run wire: the base
members below plus the FR-243 settlement `basis` and the FR-351 separating
witness record. FR-014 supplies the exact-version and closed-wire reading
rules; it does not define run-result members.

## Inputs

A clause run's disposition, package identity and usage, its FR-243 settlement basis and,
where the basis is `decisive-witness` or `decisive-counterexample`, its
FR-351 separating witness record.

## Outputs

A `native-run-result/2` record, or a located wire diagnostic.

## Behavior

`native-run-result/2` is the CLI `run` serialization of a clause run and the
run-result identifier of FR-014's registry. A producer writes it, and a reader
reads a document as a native run result only when its identifier is exactly
`native-run-result/2`; any other identifier is not a native run result and
refuses before consumption (FR-014 `unsupported-version`). A reader never
infers the identifier from payload content.

## Members

A clause-run record has exactly these top-level members: `format`
(`native-run-result/2`), `disposition`, `usage` and `basis`, plus
`package_id` when compilation completed and `witness` under the condition
below. Absent optional members are omitted, never `null`.

`disposition` is a closed object with required `stage` (`compile`, `select`,
`admit` or `evaluate`) and `category`, and the applicable optional `truth`,
`code`, `cause`, `fields`, `message` and `outcome` members. `truth` is a
Boolean; `code` and `cause` are catalog strings; `fields` maps diagnostic
field names to strings; `message` is a string. `outcome`, when present, is
one closed object tagged by `kind`: `completed_boolean` with Boolean
`value`, `completed_integer` with decimal-string `decimal`, `refused` with
no other member, `undefined` with string `reason`, or `incomplete` with
string `limit`. The applicable members and disposition categories follow
the clause-run report contract (QSL FR-109).

`usage` is a closed object with `admission` (a closed object of unsigned
`document_bytes`, `objects` and `values`), unsigned
`evaluation_admissions`, and `evaluation_consumed` (an array of closed
objects each holding string `kind` and unsigned `consumed`).

The two settlement members are:

- `basis` — the [FR-243](../../objects/protocol/FR-243-settlement-basis.md)
  settlement basis label, on every clause-run result, as every other verdict
  carries it. An evaluated `success` or `violation` carries
  `decisive-counterexample`, `decisive-witness` or `closed-scope`. Every other
  disposition (refused, undefined, incomplete, cancelled, unsupported) carries
  `unavailable`.
- `witness` — the FR-351 separating witness record, present exactly when
  `basis` is `decisive-witness` or `decisive-counterexample`, and absent
  otherwise. It is a closed object with these members:

  | Member | Value |
  | --- | --- |
  | `quantifier` | the deciding quantifier's occurrence key: an object with exactly `node_id` (the FR-322 `NodeId` shape), `role` and `ordinal`; absent for a family whose decision has no quantifier occurrence |
  | `deciding_element` | the deciding element, in the canonical typed value encoding FR-331 canonical assignments use |
  | `index` | the element's zero-based position as an unsigned integer; absent for a family that assigns no position |
  | `value_path` | the [FR-207](../../objects/foundation/FR-207-runtime-value-path.md) `runtime_value` wire form |
  | `trace_position` | the unsigned trace position; absent for a family that retains no trace |

  An absent component is an absent member, never `null`, zero or an empty
  value. The document encodes as RFC 8785 JSON.

A reader refuses a `basis` outside the FR-243 labels, a `witness`
present with a basis that is not decisive or absent with one that is, and a
`witness` missing a component its family assigns, carrying a member outside
the table, or substituting a default for an absent component.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-352-AC-1 | A clause run's result is written as `native-run-result/2` carrying the base members defined here, `basis` and, with a decisive basis, `witness`, and the strict reader admits it. | Test |
| FR-352-AC-2 | A document whose identifier is not exactly `native-run-result/2`, including a document with no identifier, refuses before consumption (FR-014 `unsupported-version`), whatever its payload holds. | Test |
| FR-352-AC-3 | `witness` is present exactly when `basis` is `decisive-witness` or `decisive-counterexample`, and absent otherwise. | Test |
| FR-352-AC-6 | Every `native-run-result/2` clause-run result carries `basis`: `decisive-counterexample` with a `witness` for a state `forall` refuted at an element, `decisive-witness` with a `witness` for an `exists` satisfied at an element, `closed-scope` with no `witness` for a truth decided elsewhere, and `unavailable` with no `witness` for a refused, undefined, incomplete, cancelled or unsupported result. | Test |
| FR-352-AC-7 | A `witness` serializes with exactly the members `quantifier`, `deciding_element`, `index`, `value_path` and `trace_position` its family assigns; a reader refuses an unknown `basis`, a `witness` whose presence disagrees with the basis, a missing assigned member, an extra member, and a `null` or default standing for an absent member. | Test |
| FR-352-AC-8 | A clause-run document with the declared closed top-level, disposition and usage members reads; independently omitting a required member, adding an unknown member at any level, or encoding an absent optional member as `null` refuses with a located wire diagnostic. | Test |

## Dependencies

- [FR-014](FR-014-read-exact-wire.md) supplies the closed-wire contract and
  the identifier refusal.
- [FR-243](../../objects/protocol/FR-243-settlement-basis.md) supplies the
  `basis` labels.
- [FR-351](../../objects/protocol/FR-351-separating-witness-record.md) supplies
  the record `witness` carries, and
  [FR-207](../../objects/foundation/FR-207-runtime-value-path.md) its value
  path.
- [US-011](../../usecase/US-011-analyze-complete-models.md) supplies the
  replay/comparability need this wire serves.

## References

- Linear STD-144; QSL ADR-031 SW-10, SW-11, R-1 to R-4.
- TC-255 exercises the native run-result wire.
