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
  compares) or a named parameter or result binding of one declared invocation
  ([FR-046](../../functional/foundation/FR-046-validate-state-invocation-inputs.md):
  "Parameters come from one declared invocation"). Two subjects of different
  kinds are never equal (FR-047).
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

  This order is **total over every set of paths sharing one observation
  identity**, which is the set a single diagnostic reports. It is **not total
  across observations**, because no requirement fixes a key for a selected
  observation and FR-265 therefore supplies no key order; see the open gap. No
  order is supplied here to close that. It carries no causal, severity or
  precedence meaning.
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

## Serialization is out of scope

FR-047 fixes the structure, equality and order of a runtime value path and fixes
no serialized spelling, discriminator tag or separator for it. This artifact
establishes none either. The existing shared interchange contract remains the
only authority on the wire, and FR-047 creates no competing result envelope.

## Location kinds are distinct

A runtime value path, an authored source span
([FR-203](FR-203-source-region.md)) and a JSON field/index path remain different
location kinds and are not compared to one another
([FR-274](FR-274-diagnostic-location-kind.md)). This artifact types the runtime
kind only. The JSON field/index path remains untyped, because no accepted
requirement states its structure.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-207-AC-1 | A field selection and a member selection carrying the same name compare unequal. | Analysis |
| FR-207-AC-2 | An option-value step and a field step at the same position compare unequal, and neither substitutes for the other. | Analysis |
| FR-207-AC-3 | Two paths with equal steps under different subjects, or under different selected observations, compare unequal. | Analysis |
| FR-207-AC-4 | A path with an empty step sequence denotes its subject and does not compare equal to an absent path. | Analysis |
| FR-207-AC-5 | A proper prefix orders before every path that extends it, and the order is used for collection comparison only. | Analysis |
| FR-207-AC-6 | Paths sharing one observation identity are totally ordered; this artifact supplies no order across observations. | Inspection |
| FR-207-AC-7 | This artifact declares no serialized spelling, discriminator tag or separator for a runtime value path. | Inspection |

## Open gap

The JSON field/index path kind remains unresolved. No accepted requirement
states its root, its step kinds, its equality or an order over it, and this
artifact supplies none; only the proposed catalog names the kind, as the
retained payload of the `invalid_package` and `invalid_runtime_input` families.
FR-274 continues to record that gap.

No requirement fixes a wire spelling for the three step kinds or for the two
subject kinds, and FR-047 explicitly leaves serialization open. An
implementation choosing tags therefore chooses them outside this specification.

**The order is total only within one observation.** FR-265 records that "the
prose does not fix the key components" of a selected snapshot and that no key
order over snapshots is stated. Root comparison therefore resolves fully for
paths sharing an observation and does not resolve between observations. Closing
that needs a requirement fixing the selected observation's key, which is
workstream F's, not this artifact's. Ordering observations by a capture
timestamp would supply an order the prose does not state, which FR-113 forbids.

## Dependencies

- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  defines the path root, the three step kinds, equality, the total order,
  absence, and the exclusion of serialization.
- [FR-204](FR-204-declared-object-identity.md) supplies one of the two subject
  kinds; [FR-046](../../functional/foundation/FR-046-validate-state-invocation-inputs.md)
  supplies the other and establishes that an absent input remains unavailable
  "without fabricating a delta, violation or Boolean" (FR-046-AC-4).
- [FR-265](../observation/FR-265-selected-snapshot.md) types the selected
  observation component of the root and records that its key is unfixed.
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

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
