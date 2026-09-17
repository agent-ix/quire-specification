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
---
# FR-207: Runtime value path

## Description

When a producer locates a value inside a validated runtime subject, the producer SHALL express that locus as a root and an ordered sequence of typed traversal steps rather than as a rendered path string.

## Properties

- **root.observation** (`selected observation identity`) — the observation the
  value is read under, typed for a snapshot by
  [FR-265](../observation/FR-265-selected-snapshot.md). It is part of the root
  and never a step, because presence facts "never cross observation identity"
  ([FR-042](../../functional/foundation/FR-042-select-pre-state-reads.md)) and a
  guard cannot justify "a same-named field on another object or observation"
  ([FR-045](../../functional/foundation/FR-045-preserve-control-and-presence-facts.md)-AC-3).
- **root.subject** (`declared object identity` | `invocation binding`) — exactly
  one of a declared object identity
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
  parameter binding. Two subjects of different kinds are never equal
  (FR-047).
- **steps** (ordered sequence of `traversal step`, possibly empty) — the
  traversal from the subject to the located value. An empty sequence denotes the
  subject itself.

Each traversal step is exactly one of three kinds (FR-047):

| Step kind | Carries | Selects |
| --- | --- | --- |
| field selection | the exact declared field spelling | a declared field of the current value |
| option-value selection | no name | the contained value of an option |
| member selection | the exact declared member spelling | a declared model member |

A field selection and a member selection carrying the same name are never equal
to one another, and an option-value selection is never a field of the option it
traverses (FR-047). The exact declared spelling is the one retained by
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

## Identity and comparison

- **Identity kind** — *value*. A runtime value path has no identity of its own
  and is compared componentwise.
- **Key** — (`root.observation`, `root.subject`, `steps`). Equality is
  componentwise over exactly these: roots equal componentwise, step sequences of
  equal length, and each corresponding step pair agreeing on step kind and on
  that kind's name where it carries one (FR-047).
  Rendered path text, display name, message wording, locale and reported source
  span never participate — FR-047-AC-1 requires that changing or localizing
  human-readable messages leaves the same typed failure's stage, code and cause
  details unchanged, and the proposed catalog states that changing "display
  paths" does not change a checked clause's static meaning.
- **Key order** — for collection comparison only: by root first, then step by
  step. Root comparison is by observation key, then by subject kind ordering a
  declared object identity before an invocation binding, then by subject key
  (FR-204's is lexicographic over `universe`, `type`, `object_identity`). Step
  comparison orders a proper prefix before every path that extends it, orders
  differing step kinds as field before option-value before member, and orders
  steps of equal kind by their exact name. The subject-kind and step-kind orders
  are fixed by FR-047 and are not asserted here.

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
- **Digest domain** — a runtime value path carries no digest. Where its root's
  declared object identity is carried beside one, that digest's domain is
  selected under [FR-201](FR-201-canonical-identity-domain.md).

## Canonical serialization

The wire form is a tagged object with `kind: runtime_value`, a typed `root`, and
an ordered `steps` array. Root subject tags are `object` and `invocation`; an `invocation` subject
carries its invocation and a binding tag `parameter` (with the exact parameter
spelling), `result` or `receiver`.
Step tags are `field`, `option_value`, `member`, `index` and `map_key`; named
steps carry exact declaration spelling, indices are unsigned decimal integers,
and map keys carry their canonical typed value. Unknown tags refuse.

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

## Resolved path boundary

JSON paths use `kind: json`, a document identity and ordered `field`/`index`
steps. Runtime and JSON paths therefore share no root or discriminator. Selected
observations compare by their authority-qualified observation identity and
revision, so runtime paths have a total lexicographic order across observations
without using capture timestamps. The canonical tags above close the wire
spelling; display renderings remain non-normative.

## Dependencies

- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  defines the path root, the three step kinds, equality, the total order,
  absence, and the exclusion of serialization.
- [FR-204](FR-204-declared-object-identity.md) supplies one of the two subject
  kinds; [FR-046](../../functional/foundation/FR-046-validate-state-invocation-inputs.md)
  supplies the other and establishes that an absent input remains unavailable
  "without fabricating a delta, violation or Boolean" (FR-046-AC-4).
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

Accepted complete-V1 runtime-path contract. Implementation and qualification
remain tracked separately.
