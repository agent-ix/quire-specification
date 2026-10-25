---
id: FR-216
title: "Channel premise"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-054
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
# FR-216: Channel premise

## Description

When a conformance result is derived, the result SHALL identify the exact premises under which it was derived, so that changing any channel premise changes the conformance request identity and its result provenance.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Sender | FR-054 | The exact authored sending role of the channel. Missing or conflicting refuses admission. |
| Receiver | FR-054 | The exact authored receiving role of the channel. Missing or conflicting refuses admission. |
| Message type | FR-054 | The one exact admitted message type carried by the channel. Missing or conflicting refuses admission. |
| Ordering policy | FR-054 | Exactly `unordered`, or `fifo` with its admitted declaration-local binder and typed key-expression definition. |
| Delivery policy | FR-054 | Exactly one of `unknown`, `at-most-once`, `at-least-once` and `exactly-once-premise`. |

Synchronized clocks are likewise never inferred (FR-054).

## Identity and comparison

- **Identity kind** — *value*. A premise set has no identity of its own. It is
  compared componentwise over the five independent components above.
- **Key** — sender, receiver, message type, ordering policy and delivery
  cardinality. For `fifo`, the ordering component includes the qualified
  protocol-definition identity, declaration-local message-binder handle and
  declaration-local typed key-expression handle; its admitted expression
  retains its complete selected type and semantic dependencies. The runtime
  value produced by evaluating that expression does not identify the static
  premise. Duplication and loss permission are derived from the delivery policy and are
  not an additional component. Equality is componentwise over exactly these
  values. Display name, external path, span, locale and rendering do not extend
  this key (FR-035, FR-049-AC-7, FR-047).
- **Key order** — componentwise, in the component order FR-054 states them:
  sender, then receiver, then message type, then ordering policy, then delivery
  cardinality. This order exists only to compare and
  diff collections of premise sets. It carries no causal meaning, no precedence
  between premises, and no claim that one premise is checked before another.
  The component order is taken from the FR-054 sentence and is not a priority.
- **Causal order** — a premise carries none. An ordering premise is a
  conformance assumption about a channel, not a causal edge: causal order is the
  authored partial acyclic order of FR-052, which admits no edge derived from
  source order or timestamps (FR-052, FR-052-AC-2). A FIFO premise therefore
  never contributes a causal edge and never orders messages outside its declared
  channel and evaluated key scope (FR-054-AC-2). A runtime FIFO partition is
  keyed by (`static channel key`, `exact typed expression result`). Equal result
  values on different channels never create order between those channels.
- **Absence** — an absent premise compares only to absence, and never to a
  default: "The linker SHALL NOT infer exactly-once delivery, FIFO, reliable
  delivery or synchronized clocks" (FR-054), and a missing required component
  refuses admission. `unordered` is a present ordering member with a positively
  absent FIFO expression; it is not a default for a missing ordering policy.
- **Digest domain** — every digest carried by a referenced static role, message
  type, FIFO expression definition or qualified protocol-definition identity
  retains its selected FR-201 domain. Equal bytes crossing domains are unequal;
  no domain is inferred from result provenance.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-216-AC-1 | Two premise sets equal in all five independent components compare equal, while changing any component, FIFO expression definition or delivery endpoint makes them unequal. | Analysis |
| FR-216-AC-2 | A conformance result names the exact premise set under which it was derived. | Inspection |
| FR-216-AC-3 | An absent premise compares only to absence and is never equal to `unordered`, FIFO, a delivery policy or a synchronized-clock fact. | Analysis |
| FR-216-AC-4 | The declared key order over premise components is used only for collection comparison and is never cited as precedence or as a causal edge. | Analysis |
| FR-216-AC-5 | `unordered` and FIFO are the complete ordering vocabulary; FIFO requires an exact admitted binder and stable-equality typed expression, and its runtime scope is one static channel plus one exact evaluated value. | Test (TC-054) |
| FR-216-AC-6 | The delivery policy alone derives whether a duplicate delivery or a loss is permitted, and no separate duplication premise is accepted. | Test (TC-054) |

## Dependencies

- [FR-054](../../functional/protocol/FR-054-bind-channel-premises.md) fixes the premise
  components, the non-inference rules and the identity effect of changing one.
- [FR-215](FR-215-protocol-channel.md) is the channel that contains this premise
  set.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) qualifies the FIFO
  binder and typed key-expression handles.

## Status

Proposed closure for specification issue #31. Spec Language already emits the
five independent channel fields and validates the FIFO binder/type/expression
and interval shape. Protocol's current position-only FIFO fingerprint is partial
and is not evidence of exact semantic identity; executable consumer evidence
remains planned under TC-054.
