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

When a native temporal clause names an operator under a bounded temporal profile, the reader SHALL resolve it through the closed vocabulary listed here, every member of which carries an inclusive interval under one selected temporal profile.

FR-091 states that under a bounded temporal profile
"the evaluator shall refuse an unbounded interval",
and FR-092 states for the past that
"the evaluator shall refuse an unbounded past interval"
and that "Implicit history reconstruction is refused under every profile."

The unbounded forms of these operator labels belong to the infinite-trace
profile and are not members of this table. FR-091 states that under that profile
"the evaluator shall admit the unbounded always, eventually, until and release operators"
with the meaning FR-161 states, and FR-092 states the same for the unbounded
historically, once, since and triggered operators. The members of this table are
also admitted under the infinite-trace profile, nested with the unbounded
forms; their key then carries that profile identity
([FR-367](../../functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)).
[FR-161](../../functional/temporal/FR-161-analyze-infinite-trace-properties.md)
owns their meaning.

## Values

| Value | Description |
| --- | --- |
| eventually | Wire spelling `eventually[a,b]`. Future. FR-091: "Some offset `d` in `[a,b]` makes `p` true at `i+d`". Existential. |
| always | Wire spelling `always[a,b]`. Future. FR-091: "Every offset `d` in `[a,b]` makes `p` true at `i+d`". Universal. |
| until | Wire spelling `until[a,b]`. Future, order-sensitive. FR-091: "Some `d` in `[a,b]` makes `q` true at `i+d`, and every `e` in `[a,d)` makes `p` true at `i+e`". FR-091 records the convention as deliberate: "positions before `a` do not satisfy its left operand." |
| release | Wire spelling `release[a,b]`. Future, order-sensitive. FR-091: "The Boolean dual of `not p until[a,b] not q` under the same profile". |
| once | Wire spelling `once[a,b]`. Past. FR-092: "Some offset `d` in `[a,b]` makes `p` true at `i-d`". Existential. |
| historically | Wire spelling `historically[a,b]`. Past. FR-092: "Every offset `d` in `[a,b]` makes `p` true at `i-d`". Universal. |
| since | Wire spelling `since[a,b]`. Past, order-sensitive. FR-092: "Some `d` in `[a,b]` makes `q` true at `i-d`, and every `e` in `[a,d)` makes `p` true at `i-e`". |
| triggered | Wire spelling `triggered[a,b]`. Past, order-sensitive. FR-092: "The Boolean dual of `not p since[a,b] not q` under the same profile". |

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
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). The profile is
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
  only to absence. An unbounded interval is not an absent bound: FR-091-AC-6
  holds that "Where a bounded temporal profile is selected, unbounded intervals
  return a non-Boolean disposition", and an unbounded operator admitted under
  the infinite-trace profile is a member of the FR-161 vocabulary rather than a
  member of this table with an absent bound.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-254-AC-1 | Each member's value equals the exact operator label in the table, and no member outside the table is admitted. | Test |
| FR-254-AC-2 | `strong previous` resolves to `once[1,1]` and is not represented as a distinct member. | Test |
| FR-254-AC-3 | The same operator label and interval under two different profile identities compares unequal. | Test |
| FR-254-AC-4 | Exactly `until[a,b]`, `release[a,b]`, `since[a,b]` and `triggered[a,b]` are recorded as order-sensitive. | Test |
| FR-254-AC-5 | Where a bounded temporal profile is selected, an unbounded interval is refused rather than recorded as an absent bound; where the infinite-trace profile is selected, the unbounded operator resolves through FR-161 rather than as a member of this table carrying an absent bound. | Test |

## Dependencies

- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) establishes the
  four future members, the lower-bound convention and pointwise connectives.
- [FR-092](../../functional/temporal/FR-092-evaluate-bounded-past.md) establishes the four
  past members and strong previous.
- [FR-090](../../functional/temporal/FR-090-select-temporal-profile-and-clock.md)
  establishes the order-sensitive subset.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Truth boundary

FR-091 and FR-092 emit the FR-242 source spellings `satisfied`, `violated` and,
for an undecided open future only, `pending`. A non-completed evaluator outcome
carries no truth at the evaluator boundary; the canonical result adapter
explicitly supplies FR-242 `unavailable`. A TL Boolean is a separately selected
mapping view and is not a source truth label.

## Status

Proposed truth-boundary reconciliation for `quire-specification#40`.
