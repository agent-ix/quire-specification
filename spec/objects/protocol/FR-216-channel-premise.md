---
id: FR-216
title: "Channel premise"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-054
    type: specifies
---
# FR-216: Channel premise

## Description

When a conformance result is derived, the result SHALL identify the exact premises under which it was derived, so that changing any channel premise changes the conformance request identity and its result provenance.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Sender | FR-054 | The authored sending participant of the channel. Missing or conflicting refuses admission (FR-054-AC-3). |
| Receiver | FR-054 | The authored receiving participant of the channel. Missing or conflicting refuses admission (FR-054-AC-3). |
| Message types | FR-054 | The authored typed messages the channel carries. Missing or conflicting refuses admission (FR-054-AC-3). |
| Ordering policy | FR-054 | Authored; "A FIFO premise orders messages only within its declared channel and key scope" (FR-054-AC-2). FIFO order across channels is never inferred (FR-054). |
| Delivery cardinality | FR-054 | Authored; "At-least-once delivery admits a duplicate receipt while retaining the single authored message/effect obligation" (FR-054-AC-1). Exactly-once delivery and reliable delivery are never inferred (FR-054). |
| Duplication premise | FR-054 | Whether duplication can occur, authored rather than inferred. |

Synchronized clocks are likewise never inferred (FR-054).

## Identity and comparison

- **Identity kind** — *value*. A premise set has no identity of its own. It is
  compared componentwise over the six components above and nothing else.
- **Key** — all six components: sender, receiver, message types, ordering
  policy, delivery cardinality and duplication premise. FR-054 fixes exactly
  this component list in its description, and FR-054-AC-4 makes every one of
  them significant: "Changing any channel premise changes the conformance
  request identity and its result provenance." Equality is componentwise over
  these components and nothing else. Display name, source path, span, message
  text, locale and rendering never participate (FR-035, FR-049-AC-7, FR-047).
- **Key order** — componentwise, in the component order FR-054 states them:
  sender, then receiver, then message types, then ordering policy, then delivery
  cardinality, then duplication premise. This order exists only to compare and
  diff collections of premise sets. It carries no causal meaning, no precedence
  between premises, and no claim that one premise is checked before another.
  The component order is taken from the FR-054 sentence and is not a priority.
- **Causal order** — a premise carries none. An ordering premise is a
  conformance assumption about a channel, not a causal edge: causal order is the
  authored partial acyclic order of FR-052, which admits no edge derived from
  source order or timestamps (FR-052, FR-052-AC-2). A FIFO premise therefore
  never contributes a causal edge and never orders messages outside its declared
  channel and key scope (FR-054-AC-2).
- **Absence** — an absent premise compares only to absence, and never to a
  default: "The linker SHALL NOT infer exactly-once delivery, FIFO order across
  channels, reliable delivery or synchronized clocks" (FR-054), and "Missing or
  conflicting sender, receiver, message-type, order or delivery premises refuse
  admission" (FR-054-AC-3). An absent premise is not an empty premise, not
  at-most-once, not unordered, and not an implicit option-none.
- **Digest domain** — no component of this key is a digest. Where a premise set
  is carried beside a digest in result provenance (FR-054-AC-4), that digest's
  domain is selected under `ix://agent-ix/quire-specification/FR-201`, and an
  identity crossing domains is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-216-AC-1 | Two premise sets equal in all six components compare equal, and two differing in any one component compare unequal. | Analysis |
| FR-216-AC-2 | A conformance result names the exact premise set under which it was derived. | Inspection |
| FR-216-AC-3 | An absent premise compares only to absence and is never equal to exactly-once, FIFO, reliable delivery or a synchronized-clock premise. | Analysis |
| FR-216-AC-4 | The declared key order over premise components is used only for collection comparison and is never cited as precedence or as a causal edge. | Analysis |

## Dependencies

- [FR-054](../../functional/FR-054-bind-channel-premises.md) fixes the premise
  components, the non-inference rules and the identity effect of changing one.
- [FR-215](FR-215-protocol-channel.md) is the channel that contains this premise
  set.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

1. FR-054-AC-3 names five premises that refuse when missing or conflicting —
   "sender, receiver, message-type, order or delivery" — while the FR-054
   description binds six, adding the duplication premise. Whether a missing
   duplication premise refuses admission is unstated. This artifact keeps all
   six in the key, because FR-054-AC-4 makes "any channel premise" significant
   to identity, and records the disagreement rather than resolving it.
2. No requirement states the value domains of the ordering policy, the delivery
   cardinality or the duplication premise. "At-least-once" (FR-054-AC-1) and
   "FIFO" (FR-054-AC-2) are named as examples; "exactly-once", "reliable" and
   "synchronized clocks" are named only as things not to infer (FR-054). No
   closed vocabulary is established, so none is enumerated here.
3. FR-054-AC-2 scopes a FIFO premise to a "key scope" that no requirement
   defines. This artifact does not treat that scope as a component of this key.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
