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
  - target: ix://agent-ix/quire-spec-language/FR-115
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-414
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-267
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-330
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-031
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
FR-351 separating witness record; for a temporal selection, its QSL FR-330
temporal report.

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
`package_id` when compilation completed, `witness` under the condition
below and `extraction` when the run's unit is an extracted Markdown clause
body. Absent optional members are omitted, never `null`.

The producer SHALL write `extraction` exactly when the clause run's unit is
the body that QSL FR-030 extracted from an original Markdown document (QSL
FR-031). QSL FR-031 owns the meaning of the record it carries. `extraction`
is a closed object with exactly these members, every string a JSON string
and every offset an unsigned JSON integer:

| Member | Value |
| --- | --- |
| `original` | closed object: `authority` and `identity`, the original document's identity labels; `path`, its authored path |
| `body` | closed object: `authority` and `identity`, the body's native identity; `document`, its formal identity |
| `binding` | closed object: `clause`, the authored clause id (the Quire heading id); `package`, the authored package |
| `outcome` | closed object: `availability`, `clauses`, `clause_text` and `diagnostics`, Quire's extraction outcome members in Quire's own encoding, unchanged; `clauses` is omitted when Quire reports no clause population |
| `map` | closed object: `region`, the body's original region as `{start, end}` byte offsets; `segments`, a non-empty array of closed objects `{body_start, body_end, original_start, original_end}` in ascending `body_start` order |
| `original_regions` | present exactly for a refusal whose locus names a body span: a non-empty array, in original-document order, of closed regions `{start, end}`, each endpoint a closed object `{byte, line, column}` |

The strict reader SHALL restore it without losing any member and SHALL refuse
an `extraction`, or a nested object of it, with an unknown, missing,
duplicate or `null` member, with a value of another JSON type, or with an
empty `segments` or `original_regions` array.

`disposition` carries QSL FR-109's clause-run disposition without loss. Its
`stage` and `category` are required; its applicable `truth` and refusal
record are retained. A `compile`, `select` or `admit` refusal retains its
FR-109 `RefusalRecord`, including `code`, `cause` and `locus`. An `evaluate`
refusal, unsupported or incomplete result retains FR-100's exact `outcome`
vocabulary, including `code`, `cause`, `fields`, `locus` and `location` when
that outcome assigns them; its `refused` form is not an empty tag. An
`evaluate` internal failure carries `internal_fault` as a closed object
with the FR-100 `InternalFault`'s string `stage` and `invariant`, and no
`outcome` member. The QSL FR-100 and FR-109
contracts determine which of these members are present for each category;
the `/2` writer and reader SHALL preserve them and SHALL NOT replace an
assigned member with `null`, a default or a generic refusal tag.

For an evaluated QSL FR-115 `Frame` violation, `disposition.refusal`
additionally carries `frame_witness`: the complete FR-115 `FrameWitness`
in QSL FR-267's closed encoding. This member is present exactly for that
case, even though the disposition category is `violation`; the enclosing
refusal's code and cause remain `frame_violation`/`unauthorized-change`.
The frame witness is distinct from the top-level FR-351 `witness`, which
is absent when the frame verdict has `basis: closed-scope`. The strict
reader SHALL restore the FR-115 witness without losing any member and
SHALL refuse a missing, malformed or misplaced `frame_witness`. FR-115
owns its fields and change variants; this requirement adds none.

`usage` is a closed object with `admission` (a closed object of unsigned
`document_bytes`, `objects` and `values`), unsigned
`evaluation_admissions`, and `evaluation_consumed` (an array of closed
objects each holding string `kind` and unsigned `consumed`).

The two settlement members are:

- `basis` — the [FR-243](../../objects/protocol/FR-243-settlement-basis.md)
  settlement basis label, on every clause-run result, as every other verdict
  carries it. An evaluated `success` or `violation` carries
  `decisive-counterexample`, `decisive-witness` or `closed-scope`; a temporal
  `undefined` result is an evaluated `violation` and carries `closed-scope`
  (Temporal disposition, below). A temporal `pending` result, category
  `inconclusive`, carries `unsettled`. Every other disposition (refused,
  undefined, incomplete, cancelled, unsupported) carries `unavailable`.
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

## Temporal disposition

A stage `evaluate` disposition of a temporal selection (QSL FR-330) whose
result is a temporal verdict, a refused `holds` condition or a trace refusal
carries `disposition.temporal`: a closed object in QSL FR-267's encoding
whose `kind` is exactly one of the ten rows below. The member is present
exactly for that case. A stage `compile`, `select` or `admit` disposition of a
temporal selection, its internal failure (written with `internal_fault`, as
for every other selection) and every non-temporal selection carry no
`temporal`. A disposition with `temporal` carries no top-level FR-351
`witness`, no `frame_witness`, no `internal_fault` and no `refusal`; its
`outcome`, when present, is in FR-100's `outcome` vocabulary. QSL FR-267
owns the members, the `trace-refused` `variant` spellings and their payloads;
this requirement adds none.

| `kind` | Members beside `kind` | `category` | `truth` | `basis` | `outcome` |
| --- | --- | --- | --- | --- | --- |
| `holds` | none | `success` | `true` | `closed-scope` | absent |
| `violated` | `position`, `counterexample` | `violation` | `false` | `closed-scope` | absent |
| `undefined` | `position`, `counterexample` | `violation` | `false` | `closed-scope` | `undefined` with the undefined value's FR-100 reason |
| `insufficient-data` | `position`, `count`, `min_count` | `undefined` | absent | `unavailable` | `undefined` with reason `insufficient-data` |
| `pending` | none | `inconclusive` | absent | `unsettled` | absent |
| `missing-fairness-premise` | `constraint` | `unsupported` | absent | `unavailable` | `refused`, `unsupported_projection`/`missing-fairness-premise` |
| `incomplete` | none | `incomplete` | absent | `unavailable` | FR-100's `incomplete` with its limit |
| `window-limit-reached` | `limit`, `value`, `setting` | `incomplete` | absent | `unavailable` | `incomplete` with limit `max_window_values` |
| `condition-refused` | `position` | the record code's category | absent | `unavailable` | FR-100's `refused` form with the record's `code`, `cause`, `fields` and `locus` |
| `trace-refused` | `variant` and the variant's payload | the refusal code's category | absent | `unavailable` | `refused` with the refusal's catalog `code` and `cause` |

The `outcome` vocabulary admits two values beside FR-100's, each in one row
only: the undefined reason `insufficient-data`
([FR-414](../temporal/FR-414-evaluate-window-aggregates.md)'s cause, the
window holding fewer filtered values than `min_count`) only in the
`insufficient-data` row, and the incomplete limit `max_window_values`, which
names a run limit and is not an accounting counter of `usage`, only in the
`window-limit-reached` row. Every other disposition carrying either is
refused: a temporal `undefined` or `incomplete` row, and a disposition of a
non-temporal selection.

The `counterexample` member is the report's FR-364 observed-arm
`TemporalCounterexample` (`trace` `observed`, `initial_state` and
`over_binding` `null`), whose `trace_position` equals `position`.

The strict reader SHALL refuse, naming the member: a `temporal` at a stage
other than `evaluate` or on a non-temporal selection; a `temporal` together
with a top-level `witness`, a `frame_witness`, an `internal_fault` or a
`refusal`; an unknown `kind` or `variant`; a member the row does not list, or
one it lists absent or `null`; a `category`, `truth`, `basis` or `outcome`
other than the row's; and a `counterexample` that FR-364's replayer refuses
before replay or whose `trace_position` differs from `position`. Writing a
temporal disposition and reading it back SHALL restore an equal disposition,
each `trace-refused` variant and its payload included.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-352-AC-1 | A clause run's result is written as `native-run-result/2` carrying the base members defined here, `basis` and, with a decisive basis, `witness`, and the strict reader admits it. | Test |
| FR-352-AC-2 | A document whose identifier is not exactly `native-run-result/2`, including a document with no identifier, refuses before consumption (FR-014 `unsupported-version`), whatever its payload holds. | Test |
| FR-352-AC-3 | `witness` is present exactly when `basis` is `decisive-witness` or `decisive-counterexample`, and absent otherwise. | Test |
| FR-352-AC-6 | Every `native-run-result/2` clause-run result carries `basis`: `decisive-counterexample` with a `witness` for a state `forall` refuted at an element, `decisive-witness` with a `witness` for an `exists` satisfied at an element, `closed-scope` with no `witness` for a truth decided elsewhere, and `unavailable` with no `witness` for a refused, undefined, incomplete, cancelled or unsupported result. | Test |
| FR-352-AC-7 | A `witness` serializes with exactly the members `quantifier`, `deciding_element`, `index`, `value_path` and `trace_position` its family assigns; a reader refuses an unknown `basis`, a `witness` whose presence disagrees with the basis, a missing assigned member, an extra member, and a `null` or default standing for an absent member. | Test |
| FR-352-AC-8 | A clause-run document with the declared top-level members and the FR-100/FR-109 disposition and usage reads. A refused outcome retains its assigned `code`, `cause`, `fields`, `locus` and `location`; an internal failure carries `internal_fault` with stage and invariant and no outcome. Independently dropping an assigned member, adding an unknown member, or encoding an absent optional member as `null` refuses with a located wire diagnostic. | Test |
| FR-352-AC-9 | A QSL FR-115 Frame violation carries `disposition.refusal.frame_witness`, retaining its complete typed invocation, pre and post selections, population, change and checked permission under QSL FR-267's encoding. A relationship link change is carried as the `link` change, which identifies the link by the relationship's `DeclarationKey`, its `difference` (`added` or `removed`), its `source` and `target` references and the frame's `modifies`, with `population` the source end's; a link that FR-106 check 11.5 leaves out carries no `frame_witness`. The report has `basis: closed-scope` and no top-level FR-351 `witness`. The strict reader restores it, and refuses a missing, malformed or misplaced `frame_witness`, and a `link` change missing a member or with a `difference` other than `added` or `removed`, with a located wire diagnostic. | Test |
| FR-352-AC-10 | Each of the ten `disposition.temporal` kinds is written at stage `evaluate` of a temporal selection with exactly its row's members, `category`, `truth`, `basis` and `outcome`, no top-level `witness` and no `refusal`, and reads back to an equal disposition; a `trace-refused` disposition with `variant` `empty-loop` and one with `variant` `tuple-arity`, `expected` 2 and `found` 1 each read back to the same variant and payload. | Test (TC-255) |
| FR-352-AC-11 | A `pending` disposition reads with category `inconclusive`, no `truth` and `basis` `unsettled`; an `undefined` disposition reads with category `violation`, `truth` `false` and `basis` `closed-scope`; an `insufficient-data` disposition reads with `outcome` `{"kind": "undefined", "reason": "insufficient-data"}`; a `window-limit-reached` disposition reads with `outcome` `{"kind": "incomplete", "limit": "max_window_values"}`; the reason `insufficient-data` in an `undefined` disposition, the limit `max_window_values` in an `incomplete` disposition, and either on a non-temporal state clause run each refuse. | Test (TC-255) |
| FR-352-AC-12 | Independently: a `temporal` on a stage `admit` disposition, on a non-temporal clause run, or beside a top-level `witness`, an `internal_fault` or a `refusal`; `kind` `held`; `variant` `empty-hoop`; a `violated` disposition without `counterexample`, with `position` `null`, or with an extra member; a `counterexample` whose `trace_position` differs from `position`; and a `pending` disposition with `basis` `unavailable` or with `truth` each refuse with a located wire diagnostic naming the member. | Test (TC-255) |
| FR-352-AC-13 | A clause run over an extracted Markdown clause body (QSL FR-031) is written with an `extraction` member under QSL FR-267's encoding and reads back with every member equal; a clause run over a unit given as bytes carries no `extraction`. Independently, an `extraction` with an unknown top-level member `source`, one with `map` missing, one with `binding` `null`, one whose `body.document` is a number, and one whose `map.segments` is `[]` each refuse with a located wire diagnostic naming `extraction` and the member. | Test |

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
- QSL FR-115 owns the evaluated frame witness; QSL FR-267 owns its closed
  serialization inside the clause-run disposition.
- QSL FR-031 owns the meaning of the extraction record of an extracted
  clause run; this requirement fixes its member set and types, and QSL FR-267
  writes and reads it as the top-level `extraction` member.
- QSL FR-330 owns the temporal clause-run report, and QSL FR-267 the closed
  `disposition.temporal` encoding, its members and its `trace-refused`
  variants. [FR-364](../temporal/FR-364-replay-temporal-counterexamples.md)
  owns the observed-arm counterexample it carries.

## References

- Linear STD-144; QSL ADR-031 SW-10, SW-11, R-1 to R-4.
- TC-255 exercises the native run-result wire.
