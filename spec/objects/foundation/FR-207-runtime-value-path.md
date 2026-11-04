---
id: FR-207
title: "Runtime value path"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-047
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-204
    type: composes
  - target: ix://agent-ix/quire-specification/FR-265
    type: composes
  - target: ix://agent-ix/quire-specification/FR-274
    type: references
  - target: ix://agent-ix/quire-specification/FR-041
    type: references
  - target: ix://agent-ix/quire-specification/FR-351
    type: references
---
# FR-207: Runtime value path

## Description

When a producer locates a value inside a validated runtime subject, the producer SHALL express that locus as a root and an ordered sequence of typed traversal steps rather than as a rendered path string.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| root_observation | String | 1..1 |  |
| root_subject | String | 1..1 |  |
| steps | String | 0..* |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| root_observation | selected observation identity; type not stated in source; see Field sources |
| root_subject | exactly one of: declared object identity (DeclaredObjectIdentity), invocation binding, population, built collection; the text names no union type; see Field sources |
| steps | ordered sequence of traversal step, possibly empty; each step is one of six kinds; step type not stated in source; see Field sources |


- **root.observation** (`selected observation identity`) — the observation the
  value is read under, typed for a snapshot by
  [FR-265](../observation/FR-265-selected-snapshot.md). It is part of the root
  and never a step, because presence facts "never cross observation identity"
  ([FR-042](../../functional/foundation/FR-042-select-pre-state-reads.md)) and a
  guard cannot justify "a same-named field on another object or observation"
  ([FR-045](../../functional/foundation/FR-045-preserve-control-and-presence-facts.md)-AC-3).
- **root.subject** (`declared object identity` | `invocation binding` |
  `population` | `built collection`) — exactly one of a declared object identity
  ([FR-204](FR-204-declared-object-identity.md), supplying the universe, type
  and object identity that
  [FR-009](../../functional/foundation/FR-009-preserve-object-identity.md)
  compares) or a binding of one declared invocation
  ([FR-046](../../functional/foundation/FR-046-validate-state-invocation-inputs.md):
  "Parameters come from one declared invocation"). An invocation binding is
  exactly one of a named parameter binding, the result binding, or the receiver
  binding `self`. The receiver binding is the context the invocation is made
  on when that context has no declared object identity, such as a state-machine
  context ([FR-208](FR-208-quire-meaning-vocabulary.md)); a receiver with a
  declared object identity is always a declared object identity subject, never
  the receiver binding. Two invocation bindings are equal exactly when they
  name one invocation and agree on binding kind and, for a parameter, on the
  exact parameter spelling, so the receiver binding `self` never equals a
  parameter binding. Invocation bindings are key-ordered by invocation identity (the identity FR-004 selects, compared by the UTF-8 bytes of its RFC 8785 (JCS) canonical JSON serialization), then binding kind ordered parameter before result before receiver, then parameter spelling by UTF-8 bytes.
  A *population* subject is the object universe of a declared population
  ([FR-204](FR-204-declared-object-identity.md)'s universe identity), for a
  value read from a population query. A *built collection* subject is the
  occurrence key (`node_id`, `role`, `ordinal`, the FR-322 `source_map` key) of
  the expression that builds a collection inside the evaluated claim, for an
  element that has no stored location in the observation. Two subjects of
  different kinds are never equal.
- **steps** (ordered sequence of `traversal step`, possibly empty) — the
  traversal from the subject to the located value. An empty sequence denotes the
  subject itself.

Each traversal step is exactly one of six kinds:

| Step kind | Carries | Selects |
| --- | --- | --- |
| field selection | the exact declared field spelling | a declared field of a record value, or of a state-machine context ([FR-208](FR-208-quire-meaning-vocabulary.md)) |
| option-value selection | no name | the contained value of an option |
| member selection | the exact declared member spelling | a declared member of a declared object, selected from a declared object identity or from the object a reference names |
| index selection | an unsigned integer position | the element at that zero-based position of a collection value |
| map-key selection | a canonical typed key value | the value stored under that key of a map value |
| object-reference selection | the declared object identity the reference names ([FR-204](FR-204-declared-object-identity.md)) | the object a reference value refers to |

A selection from a declared object is always a member selection and never a
field selection, so a path from a declared object identity subject has a member
step where it first selects from that object, and a field step only after the
path has passed into a record value; a selection from a state-machine context
receiver is a field selection.

A field selection and a member selection carrying the same name are never equal
to one another, and an option-value selection is never a field of the option it
traverses. The exact declared spelling is the one retained by
[FR-037](../../functional/foundation/FR-037-parse-shared-native-expressions.md),
which requires that qualified model and member names "retain their exact
spelling even when the member is a keyword".

An option-value step is justified only where the option is present. An unguarded
unwrap is `undefined_expression`
([FR-005](../../functional/foundation/FR-005-guard-optional-values.md)), and a
guard justifies only "the same immutable optional value/field/observation"
([FR-045](../../functional/foundation/FR-045-preserve-control-and-presence-facts.md)).
A path therefore records where a value was sought; it does not assert that the
value was present.

## Relationship-end navigation

A runtime value path has no step kind for a relationship end. A navigation
over a declared relationship end (`e.end`,
[FR-152](../../functional/type-model/FR-152-bind-systems-model-structures.md)
"Navigation") is not a read of a stored value and is recorded as no path of its
own: the producer records no entry and no step for it, and the route taken is
never part of a path. A selection on the result of a navigation is located by a
path rooted at the declared object identity of the referenced object the
navigation returned, in the observation the navigation read, with a single
member step naming the selected member and no `reference` step, because that
identity is already the root subject. The destination end supplies the targets,
so a selection on a navigation that returns several objects is located once for
each referenced object, each at its own identity. The path locates the stored
value, not the route that reached it.

## Computed collections

A path to an element of a collection computed by `filter` or `map` names the
stored source occurrence the element was bound from, following each source
binding [FR-041](../../functional/foundation/FR-041-evaluate-ordered-queries.md)-AC-4
retains back to a stored collection, and its index step is the element's
position in that stored collection. An element of a collection built inside
the claim, which has no stored source location, is located by a path rooted at
the built collection subject, whose index step is the element's position in
the built collection. These are the value paths a separating witness record
carries ([FR-351](../protocol/FR-351-separating-witness-record.md)).

## Identity and comparison

- **Identity kind** — *value*. A runtime value path has no identity of its own
  and is compared componentwise.
- **Key** — (`root.observation`, `root.subject`, `steps`). Equality is
  componentwise over exactly these: roots equal componentwise, step sequences of
  equal length, and each corresponding step pair agreeing on step kind and on
  that kind's name, index or canonical key value where it carries one.
  Rendered path text, display name, message wording, locale and reported source
  span never participate — FR-047-AC-1 requires that changing or localizing
  human-readable messages leaves the same typed failure's stage, code and cause
  details unchanged, and the proposed catalog states that changing "display
  paths" does not change a checked clause's static meaning.
- **Key order** — for collection comparison only: by root first, then step by
  step. Root comparison is by observation key, then by subject kind ordering a
  declared object identity before an invocation binding, then by subject key
  (FR-204's is lexicographic over `universe`, `type`, `object_identity`; an
  invocation binding's is its invocation identity (the identity FR-004 selects, compared by the UTF-8 bytes of its RFC 8785 (JCS) canonical JSON serialization), then binding kind ordered parameter before result before receiver, then parameter spelling by UTF-8 bytes). Step
  comparison orders a proper prefix before every path that extends it, orders
  differing step kinds as field before option-value before member before index before map-key before object-reference, and orders steps of equal kind by their exact name, index steps numerically, map-key steps by the UTF-8 bytes of the key value's RFC 8785 (JCS) canonical JSON serialization and object-reference steps by FR-204's key. Subject kinds order declared object identity before invocation binding before population before built collection; a population subject orders by its universe key and a built collection subject by `node_id` in FR-322's componentwise node-id order, then role, then ordinal. This requirement is the one source of
  the subject kinds, the step kinds and their orders; FR-047 reports paths
  under them.

  Selected observations compare by authority-qualified observation identity and
  revision. The resulting path order is total across observations and carries no
  causal, severity or precedence meaning.
- **Causal order** — none. FR-052 constructs the causal graph "without deriving
  causal edges from source order", so a path that precedes another in key order
  is not its causal predecessor, and step position expresses containment rather
  than sequence in time.
- **Absence** — an unavailable path is explicitly absent and compares only to
  absence. An empty step sequence denotes the subject itself and is never an
  absent path, and a name-search match is never a path (FR-047, FR-047-AC-4).
  The proposed catalog states the same prohibition: "byte zero, an empty path or
  a name-search match cannot masquerade as a located failure". An absent
  observation or an absent subject yields no path at all, rather than a rooted
  path with an absent component.

## Canonical serialization

The wire form is a tagged object with `kind: runtime_value`, a typed `root`, and
an ordered `steps` array. The `root` object has exactly the members
`observation` (the selected observation identity) and `subject`. Root subject tags are `object`, `invocation`, `population` and `built`. A `population` subject is an object with exactly the members `tag` and `universe` (the FR-204 universe identity). A `built` subject is an object with exactly the members `tag` and `occurrence`, an object with exactly the members `node_id`, `role` and `ordinal`. An `invocation` subject is
an object with exactly the members `tag` (`invocation`), `invocation` (the
invocation identity), `binding` (exactly `parameter`, `result` or `receiver`)
and, exactly when `binding` is `parameter`, `parameter` (the exact parameter
spelling).
Step tags are `field`, `option_value`, `member`, `index`, `map_key` and
`reference`. Each step is an object with member `tag` and exactly one further
member where its kind carries one: `name` for `field` and `member` (the exact
declaration spelling), `index` for `index` (an unsigned decimal integer),
`key` for `map_key` (the canonical typed key value) and `object` for
`reference` (the FR-204 declared object identity); an `option_value` step has
no further member. Unknown tags refuse.

## Location kinds are distinct

A runtime value path, an authored source span
([FR-203](FR-203-source-region.md)) and a JSON field/index path remain different
location kinds and are not compared to one another
([FR-274](FR-274-diagnostic-location-kind.md)). FR-274 types all three kinds and
gives the JSON path a document identity plus ordered field/index steps. This
artifact defines only the runtime kind.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-207-AC-1 | A field selection and a member selection carrying the same name compare unequal. | Analysis |
| FR-207-AC-2 | An option-value step and a field step at the same position compare unequal, and neither substitutes for the other. | Analysis |
| FR-207-AC-3 | Two paths with equal steps under different subjects, or under different selected observations, compare unequal. | Analysis |
| FR-207-AC-4 | A path with an empty step sequence denotes its subject and does not compare equal to an absent path. | Analysis |
| FR-207-AC-5 | A proper prefix orders before every path that extends it, and the order is used for collection comparison only. | Analysis |
| FR-207-AC-6 | Runtime value paths are totally ordered across observations by the authority-qualified observation identity and revision before subject and traversal steps. | Inspection |
| FR-207-AC-7 | The canonical wire form uses `runtime_value`, typed root-subject tags and typed traversal-step tags; unknown tags refuse. | Inspection |
| FR-207-AC-8 | A path rooted at the receiver binding `self` of an invocation whose receiver has no declared object identity locates values of that receiver, compares unequal to a path with equal steps rooted at a parameter binding of the same invocation, and is never used for a receiver with a declared object identity. | Test (TC-235) |
| FR-207-AC-9 | A path to an element of a population query result is rooted at a `population` subject naming the population's universe, and a path to an element of a collection built inside the claim is rooted at a `built` subject naming the building expression's occurrence key, with an index step for the element's position in that collection; each serializes with exactly its tagged members and an extra or missing member refuses. | Test (TC-429) |
| FR-207-AC-10 | A path that follows a reference value carries a `reference` step naming the referenced object's FR-204 identity, compares unequal to a path with a field or member step at that position, and orders after a map-key step at that position. | Test (TC-429) |
| FR-207-AC-11 | A path to an element of a collection computed by `filter` or `map` names the stored source collection the element was bound from and the element's index in it, never the position in the computed collection. | Test (TC-429) |
| FR-207-AC-12 | A selection of the member `level` on the result of a relationship navigation that returns the object `t1` of type `Tank` is located at the declared object identity of `t1` under the observation the navigation read, with the single step `[member level]`, and the navigation itself records no path and no step. | Test (TC-429) |

## Resolved path boundary

JSON paths use `kind: json`, a document identity and ordered `field`/`index`
steps. Runtime and JSON paths therefore share no root or discriminator. Selected
observations compare by their authority-qualified observation identity and
revision, so runtime paths have a total lexicographic order across observations
without using capture timestamps. The canonical tags above close the wire
spelling; display renderings remain non-normative.

## Dependencies

- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  defines the path root, the five step kinds, equality, the total order,
  absence, and the exclusion of serialization.
- [FR-204](FR-204-declared-object-identity.md) supplies one of the two subject
  kinds; [FR-046](../../functional/foundation/FR-046-validate-state-invocation-inputs.md)
  supplies the other and establishes that an absent input remains unavailable
  "without fabricating a delta, violation or Boolean" (FR-046-AC-4).
- [FR-208](FR-208-quire-meaning-vocabulary.md) is the source of the receiver
  binding `self`: its State-machine context anchors bind a context with no
  declared object identity as the receiver of an invocation.
- [FR-265](../observation/FR-265-selected-snapshot.md) types the selected
  observation component of the root.
- [FR-042](../../functional/foundation/FR-042-select-pre-state-reads.md)
  establishes that presence facts never cross observation identity.
- [FR-009](../../functional/foundation/FR-009-preserve-object-identity.md)
  establishes that equal fields do not imply equal objects, so a path rooted at
  a different object is a different path.
- [FR-005](../../functional/foundation/FR-005-guard-optional-values.md) and
  [FR-045](../../functional/foundation/FR-045-preserve-control-and-presence-facts.md)
  establish that an option-value traversal requires a presence proof for that
  exact immutable value.
- [FR-037](../../functional/foundation/FR-037-parse-shared-native-expressions.md)
  establishes that member spellings are retained exactly.
- [FR-274](FR-274-diagnostic-location-kind.md) holds the three location kinds
  distinct.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.

## Status

Accepted complete-V1 runtime-path contract.
