---
id: FR-226
title: "Business effect"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-058
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-225
    type: references
---
# FR-226: Business effect

## Description

When a business observation is bound to a protocol instance, the protocol binder SHALL preserve the effect as a subject distinct from the message, the send, the receive, the delivery and the attempt, established only through an explicit authored relationship and never inferred from a transport fact ([FR-051](../../functional/FR-051-preserve-communication-identities.md)).

## Properties

- **occurrence** ([FR-221](FR-221-protocol-node-occurrence.md)) — an effect is
  an authored protocol node.
- **establishing attempt** ([FR-225](FR-225-operation-attempt.md)) — the
  concrete attempt the effect is of. The
  [choreography surface](../../../proposals/quire-v1/choreography-surface.md)
  states that `effect ... of AttemptNode` observes "A separately established
  successful business effect of that concrete attempt. A successful transport
  status or an attempt's existence is insufficient."
- **captures** — the immutable captured amounts, identifiers and anchors that
  depend on the effect. FR-055 requires captures to be "immutable for that
  obligation" and FR-055-AC-1 activates a refund obligation "with E1's captured
  amount and identity".

An effect is never inferred. FR-051 states that the binder "SHALL NOT infer a
business effect from a send, receive, successful transport status, repeated
delivery, trace identifier or equal payload alone", and FR-049-AC-2 states that
"a duplicate receipt cannot mint another effect node".

## Identity and comparison

- **Identity kind** — *derived*, through the node occurrence it is, and related
  to the concrete attempt that established it.
- **Key** — (`static declaration path`, `workflow instance identity`,
  `iteration identity`), as held by FR-221. Equality is componentwise over that
  key and nothing else. Display name, source path, span, message text, locale
  and rendering never participate. Neither does the payload nor the trace
  identifier, both excluded by FR-051 and FR-051-AC-4. Whether the establishing
  attempt is also a key component is not stated; see **Open gap**.
- **Key order** — none is established. No requirement fixes a total order over
  effects. FR-058-AC-3 states that "Two related partial refunds satisfy the
  captured amount only under a complete declared population with exact
  arithmetic", which is a membership and completeness rule over a declared
  population, not an order over its members.
- **Causal order** — the authored partial acyclic order of FR-052, carried by
  FR-220, never derived from key order, timestamp or source position
  (FR-052-AC-2, FR-113). The commit-ordering relations of FR-057-AC-3 are
  authored causal relations, not a key order.
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value. An attempt with no established effect has an absent
  effect, which is never compared to an unsuccessful effect or to an empty one
  (FR-051-AC-3).
- **Digest domain** — no component of this key is a digest; a digest carried in
  provenance carries the domain it was derived under,
  `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-226-AC-1 | An effect reference does not satisfy a comparison expecting a send, a receive, a delivery or an attempt. | Analysis |
| FR-226-AC-2 | A successful transport status, a repeated delivery, a trace identifier and an equal payload each fail to establish an effect. | Analysis |
| FR-226-AC-3 | No requirement text supplies a component tuple by which two effect claims are judged to be the same effect. | Inspection |
| FR-226-AC-4 | No requirement text supplies a total order over effects. | Inspection |

## Dependencies

- [FR-051](../../functional/FR-051-preserve-communication-identities.md)
  preserves effect as a distinct typed subject and forbids inference.
- [FR-058](../../functional/FR-058-preserve-retry-and-partial-recovery.md)
  requires each effect identity to be preserved and names the duplicate effect.
- [FR-057](../../functional/FR-057-enforce-commit-recovery.md) supplies the
  commit-ordering relations an effect participates in.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Open gap

**"Duplicate business effect" is a required outcome and no requirement states
when two effects are the same effect.** The detection depends on an equality the
prose does not define.

- FR-051-AC-2: "Relabeling a duplicate delivery as a second business effect
  refuses the binding or yields the authored duplicate-effect violation."
- FR-058: "The evaluator SHALL distinguish a repeated receipt, retried attempt
  and duplicate business effect."
- FR-059: "It SHALL report unexpected, missing, misordered, duplicated and
  wrongly correlated events without repairing them."

These two sentences describe the same duplicate effect and do not agree on its
disposition; this artifact records both and resolves neither:

- FR-051-AC-2: "refuses the binding or yields the authored duplicate-effect
  violation."
- FR-058-AC-2: "A timed-out attempt, failed attempt and duplicate effect remain
  three distinct outcomes."

A refusal of the binding and a retained distinct outcome are different results
for the same observation.

To close this gap an owner would have to decide, and a requirement would have to
state:

1. Whether the establishing attempt is a component of effect identity, so that
   two effect claims of one attempt are one effect and a second is a duplicate.
2. What makes two effect claims the same when they are of different attempts,
   given that equal payload and trace identifier are excluded by FR-051.
3. Whether a duplicate effect refuses the binding (FR-051-AC-2) or is retained
   as a distinct outcome (FR-058-AC-2).
4. Whether any order over effects exists beyond the authored causal and commit
   relations, and if one is authored, the explicit statement that it carries no
   causal meaning.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
