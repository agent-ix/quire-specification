---
id: FR-254
title: "Bounded temporal operator vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-250
    type: references
  - target: ix://agent-ix/quire-specification/FR-255
    type: references
  - target: ix://agent-ix/quire-specification/FR-242
    type: references
---
# FR-254: Bounded temporal operator vocabulary

## Description

When a native temporal clause names an operator, the reader SHALL resolve it through the closed bounded vocabulary listed here, every member of which carries an inclusive interval under one selected temporal profile.

FR-091 states that "The v1 temporal profiles shall refuse unbounded intervals",
and FR-092 states that "The v1 temporal profiles shall refuse unbounded past and
implicit history reconstruction."

## Values

| Value | Description |
| --- | --- |
| `eventually[a,b]` | Future. FR-091: "Some offset `d` in `[a,b]` makes `p` true at `i+d`". Existential. |
| `always[a,b]` | Future. FR-091: "Every offset `d` in `[a,b]` makes `p` true at `i+d`". Universal. |
| `until[a,b]` | Future, order-sensitive. FR-091: "Some `d` in `[a,b]` makes `q` true at `i+d`, and every `e` in `[a,d)` makes `p` true at `i+e`". FR-091 records the convention as deliberate: "positions before `a` do not satisfy its left operand." |
| `release[a,b]` | Future, order-sensitive. FR-091: "The Boolean dual of `not p until[a,b] not q` under the same profile". |
| `once[a,b]` | Past. FR-092: "Some offset `d` in `[a,b]` makes `p` true at `i-d`". Existential. |
| `historically[a,b]` | Past. FR-092: "Every offset `d` in `[a,b]` makes `p` true at `i-d`". Universal. |
| `since[a,b]` | Past, order-sensitive. FR-092: "Some `d` in `[a,b]` makes `q` true at `i-d`, and every `e` in `[a,d)` makes `p` true at `i-e`". |
| `triggered[a,b]` | Past, order-sensitive. FR-092: "The Boolean dual of `not p since[a,b] not q` under the same profile". |

`strong previous` is not a ninth member. FR-092 states that "The v1 profile shall
define strong previous as `once[1,1]`", that is, as a fixed instance of an
existing member. FR-092 also states that "If a later profile admits weak
previous, then that profile shall define its boundary separately from strong
previous"; no weak-previous member is supplied here, because no v1 requirement
names one.

The Boolean connectives `not`, `and`, `or` and `implies` are not members of this
vocabulary. FR-091 states that they "shall evaluate pointwise at every offset
traversed by their enclosing temporal operator" and "shall not first reduce an
operand over the entire interval and then apply one untimed Boolean operation to
that aggregate."

## Order sensitivity is a closed subset

FR-090-AC-4 names exactly four members as order-sensitive: "Equal timestamps
without an admitted causal/sequence order refuse `until`, `release`, `since` and
`triggered`; ingestion order is not substituted." Those four require the
admitted causal/sequence order of the clock binding
([FR-252](FR-252-temporal-clock-binding.md)); a key order over anchors or
timestamps never substitutes for it.

## Identity and comparison

- **Identity kind** — *value*. An operator label has no identity of its own; it
  is compared componentwise as an exact string.
- **Key** — the operator label together with its inclusive interval
  ([FR-255](FR-255-bounded-temporal-interval.md)) and the selected temporal
  profile identity ([FR-250](FR-250-temporal-semantic-profile.md)). Equality is
  componentwise over exactly those components and nothing else. Display name,
  source path, span, message text, locale and rendering never participate
  ([FR-009](../../functional/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/FR-047-emit-typed-located-causes.md)). The profile is
  a key component because FR-091-AC-2 holds that "On a one-position closed trace
  where atomic `p` is true, `always[0,1] p` is false under false-extension and
  true under a complete finite-window interpretation; the profiles cannot share
  one result identity."
- **Key order** — lexicographic over the operator label, then over the interval
  key order, for collection comparison only. It carries no precedence, no
  evaluation order and no causal meaning, and it is not an order over the
  positions or instants the operator quantifies.
- **Causal order** — none of its own. Where a member is order-sensitive, the
  order it uses is the admitted causal/sequence order of FR-252, never derived
  from key order, timestamp or ingestion order (FR-090, FR-090-AC-4).
- **Absence** — an operand or bound that is not supplied is absent and compares
  only to absence. An unbounded interval is not an absent bound: FR-091 and
  FR-092 refuse it outright, and FR-091-AC-6 holds that "Unbounded intervals,
  unavailable total valuations and exhausted semantic work return non-Boolean
  dispositions."
- **Digest domain** — not applicable; no key component of this object is a
  digest. The profile identity it names carries one, under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-254-AC-1 | Each member's value equals the exact operator label in the table, and no member outside the table is admitted. | Inspection |
| FR-254-AC-2 | `strong previous` resolves to `once[1,1]` and is not represented as a distinct member. | Inspection |
| FR-254-AC-3 | The same operator label and interval under two different profile identities compares unequal. | Analysis |
| FR-254-AC-4 | Exactly `until[a,b]`, `release[a,b]`, `since[a,b]` and `triggered[a,b]` are recorded as order-sensitive. | Inspection |
| FR-254-AC-5 | An unbounded interval is refused rather than recorded as an absent bound. | Analysis |

## Dependencies

- [FR-091](../../functional/FR-091-evaluate-bounded-future.md) establishes the
  four future members, the lower-bound convention and pointwise connectives.
- [FR-092](../../functional/FR-092-evaluate-bounded-past.md) establishes the four
  past members and strong previous.
- [FR-090](../../functional/FR-090-select-temporal-profile-and-clock.md)
  establishes the order-sensitive subset.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Open gap

The three requirements that state what applying these members yields do not use
one truth vocabulary, and none of them reconciles the others.

FR-091 Outputs:

> "A true, false or pending temporal truth disposition with the exact decision
> scope, settlement basis and decision-support identities needed by the shared
> result contract, or a separate incomplete or refused disposition with the
> affected clause, interval and missing premise."

FR-092 Outputs, for the same operators in the past direction:

> "A true or false past-time result, or a separate incomplete/refused
> disposition that identifies missing history, an unsupported profile/operator
> or a resource failure."

FR-061, which FR-091 explicitly feeds through "the shared result contract":

> "Truth is satisfied | Assessment execution is completed, activation is
> triggered and the settlement basis is `closed-scope` or `decisive-witness`."

> "Truth is violated | Assessment execution is completed, activation is
> triggered and the settlement basis is `closed-scope` or
> `decisive-counterexample`."

> "Truth is pending | Assessment execution is completed, activation is
> triggered, decision-scope progress is open and the settlement basis is
> `unsettled`."

Whether true/false/pending and satisfied/violated/pending are one vocabulary
under two spellings, and whether a past-time result can be pending at all given
that FR-092 omits pending from its Outputs, is not stated anywhere. This
artifact records all three and resolves none of them; no truth vocabulary is
enumerated here. The truth vocabulary itself is authored as
[FR-242](../protocol/FR-242-truth-disposition.md) in the result object set.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
