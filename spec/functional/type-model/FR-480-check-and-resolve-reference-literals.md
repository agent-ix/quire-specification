---
id: FR-480
title: "Check and resolve reference literals"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-339
    type: references
  - target: ix://agent-ix/quire-specification/FR-204
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-381
    type: references
---

# FR-480: Check and resolve reference literals

## Description

When a clause writes a reference literal `M::T#"k"`, the checker SHALL type it
`Reference<M::T>`, and when a subject is bound the analyzer SHALL resolve it,
once and before any evaluation, to the key with identity `k` in `T`'s
universe: a fixed object when an initial member record admits the key, and
otherwise the object an operation creates under that key. A reference literal
is the only source form that names one object by its key.

## Inputs

- A `reference-literal` node of the source tree
  ([complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md)).
- The unit's model aliases and the admitted domain packages they select.
- For resolution, a subject's universes and initial states
  ([FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md)
  "Binding of the clause's `over` parameter").

## Outputs

- A checked literal of static type `Reference<M::T>`, lowered to a
  `literal` term of `value_kind` `reference`
  ([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)).
- For a bound subject, the literal's value: an FR-181 reference
  `{"type":"reference","universe":…,"object_type":…,"identity":…}`.
- A located refusal, with no checked literal or no evaluation.

## Behavior

### Syntax

`reference-literal = qualified-name, '#', string ;` is a `primary`
alternative. `#` is a punctuation token used by this production alone. The
qualified name names the target type and the string names the key: the key's
object identity is the UTF-8 encoding of the string's Unicode scalar
sequence, with no normalization. `M::Server#"s1"` names the `Server` object
whose identity is the two bytes `s1`.

### Static checking

- The checker SHALL resolve the qualified name as a `type-ref` qualified name
  resolves. A name that resolves to no declaration SHALL refuse
  `missing_declaration`/`missing-name` at the name. A name that resolves to a
  declaration that is not a model object type (`model`/`object_type`) SHALL
  refuse `ill_typed`/`type-mismatch` at the name, retaining `expected`
  (`object type`) and `actual` (the resolved kind).
- An empty key string SHALL refuse `invalid_package`/`invalid-value` at the
  string, because no object identity is empty.
- The checker SHALL type the literal `Reference<M::T>`, the same type node a
  `Reference<M::T>` type reference names, and SHALL admit it wherever an
  expression of that type is admitted. Its upcasts, equality and
  dereference are those of any `Reference<M::T>` value.
- A fixed reference literal (Resolution at subject binding) is a constant of
  its bound subject. It is no application of a model operation, so the slots that bar model state
  ([FR-464](../../objects/interfaces/FR-464-checked-package-protocol-node-bodies.md))
  admit it, and it adds no location to a read footprint. A dereference or
  field read through it is a model-state read like any other.
- A reference literal names a key of every population that overlaps `T`
  (FR-361). [FR-381](../analysis/FR-381-check-symmetric-populations.md)
  refuses it in a unit that annotates such a population symmetric.

### Lowering

The producer SHALL lower the literal to
`{term: "literal", type: R, value_kind: "reference", value: h}`, where `R` is
the `composite_type`/`reference` node over `T`'s model declaration node and
`h` is the lowercase hexadecimal of the identity bytes, two digits per byte.
The written alias and the source spelling of the string do not enter the
term: two literals with the same type and the same identity bytes are one
term.

### Resolution at subject binding

- When a subject is bound, the analyzer SHALL resolve each reference literal
  of the checked package once, before any state is explored or any clause is
  evaluated. The universe component is `T`'s object universe
  ([FR-204](../../objects/foundation/FR-204-declared-object-identity.md)), and
  the key is the pair of that universe and the literal's identity.
- A literal whose key occurs in the universe of no population that overlaps
  `T` (FR-361) SHALL refuse `missing_declaration`/`missing-name` at the
  literal, naming `T` and the key as its hexadecimal identity, and nothing over
  the subject is evaluated.
- When a member record of some initial state admits the key, as FR-361 admits
  an instance key, the literal is **fixed**. Its type component is the type
  that record fixes, which SHALL conform to `T`, else the same
  `missing-name` refusal applies. A fixed literal evaluates to the same FR-181
  reference at every state of every behaviour. When its object is not a
  member of a state's population, a dereference or field read through it
  follows the rules for an absent target, as any stored reference to a
  deleted object does.
- When the key occurs in an overlapping population's universe and no initial
  member record admits it, so only an operation can create its object, the
  literal is **created-object**. At each state it evaluates to the FR-181
  reference of that state's member with that key, its type component being
  the type the member's record fixes. It evaluates to the undefined value with
  reason `absent-key` at a state with no such member, or with a member whose
  type does not conform to `T`. A created-object literal reads the membership
  of the populations that overlap `T`, so it is a model-state read: its read
  footprint holds that membership. In a slot that bars model state it
  SHALL refuse `ill_typed`/`operator-ineligible` at the literal when the
  subject is bound, before any state is explored, as such a slot refuses a
  `lookup`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-480-AC-1 | `M::Server#"s1"` parses as one `reference-literal` node holding the qualified name and the string, and renders back byte for byte. `M::Server#s1`, `M::Server#` and `#"s1"` each fail at the parser with a located syntax diagnostic. | Test (TC-965) |
| FR-480-AC-2 | `M::Server#"s1"` types `Reference<M::Server>`, with the same type node as a `Reference<M::Server>` parameter, and `self = M::Server#"s1"` in an invariant of `Server` checks. `M::Absent#"s1"` refuses `missing_declaration`/`missing-name` at `M::Absent`; `M::Point#"s1"`, `Point` a record type, refuses `ill_typed`/`type-mismatch` at `M::Point`, retaining expected `object type` and actual `record`; `M::Server#""` refuses `invalid_package`/`invalid-value` at the string. | Test (TC-965) |
| FR-480-AC-3 | `M::Server#"s1"` lowers to a `literal` term of `value_kind` `reference`, `type` the `composite_type`/`reference` node over `Server`, and `value` `7331`. Two occurrences of it, one written through a second alias of the same package, share the term. | Test (TC-965) |
| FR-480-AC-4 | Over a subject whose `servers` universe in `Server`'s object universe is `[s1, s2, s3]`, whose initial state admits `s1` and `s2` as `Server` records and whose operation `spawn()` creates `s3` as a `Server`, `M::Server#"s1"` resolves to the FR-181 reference with that universe, `Server`'s type identity and identity `7331`, and `self = M::Server#"s1"` holds for `s1` only. `M::Server#"s9"`, whose key is in no universe, refuses `missing_declaration`/`missing-name` at the literal, naming `Server` and `7339`, before any state is explored. `M::Server#"s3"`, whose key is in the universe and admitted by no initial record, is created-object: it is undefined with reason `absent-key` at the initial state, and after an operation that creates `s3` as a `Server` it evaluates to `s3`'s reference. | Test (TC-965) |
| FR-480-AC-5 | A choice guard that compares a `receive` record's reference field to `M::Server#"s1"` checks: the fixed literal is not a model-state read, while the same guard over the created-object `M::Server#"s3"` refuses `ill_typed`/`operator-ineligible` at the literal when the subject is bound. A guard that reads `deref(M::Server#"s1").load` refuses `ill_typed`/`operator-ineligible` at the dereference, as any model-state read there does. | Test (TC-965) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md): the
  `reference-literal` production.
- [FR-204](../../objects/foundation/FR-204-declared-object-identity.md): the
  identity triple and the object universe of a type.
- [FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md):
  admitted keys and overlapping populations.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md): the reference
  value form.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md): the
  `reference` literal `value_kind`.
- [FR-381](../analysis/FR-381-check-symmetric-populations.md): the
  symmetric-population refusal of a literal that names a key.
