---
id: FR-051
title: "Preserve message, delivery, attempt and effect identities"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-050
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-210
    type: references
  - target: ix://agent-ix/quire-specification/FR-213
    type: references
  - target: ix://agent-ix/quire-specification/FR-214
    type: references
  - target: ix://agent-ix/quire-specification/FR-222
    type: references
  - target: ix://agent-ix/quire-specification/FR-223
    type: references
  - target: ix://agent-ix/quire-specification/FR-224
    type: references
  - target: ix://agent-ix/quire-specification/FR-225
    type: references
  - target: ix://agent-ix/quire-specification/FR-226
    type: references
  - target: ix://agent-ix/quire-specification/FR-235
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
## Description

When binding communications and business observations to a protocol instance, the protocol binder SHALL preserve message, send, receive, delivery, attempt and effect as distinct typed subjects.

## Inputs

A linked protocol, typed message/channel declarations and F-owned observation
bindings with explicit business relationships and provenance.

## Outputs

Bound protocol events and effects, or typed identity/correlation refusals and
explicitly missing-observation dependencies.

## Behavior

The binder SHALL require explicit relationships between transport records,
attempts and effects. The binder SHALL NOT infer a business effect from a send,
receive, successful transport status, repeated delivery, trace identifier or
equal payload alone. The binder SHALL retain source and observation identities
for every accepted correspondence.

For a concrete runtime subject, the binder SHALL use one FR-035
authority-qualified opaque reference of the exact distinct kind `delivery`,
`operation-attempt` or `business-effect`. A delivery SHALL bind separately to
its workflow instance, static channel, delivery requirement and related send
occurrence. A forward operation attempt SHALL bind separately to its protocol
node occurrence, role and operation requirement. A business effect SHALL bind
separately to its effect node, effect requirement and establishing attempt.

The producer-local component is retained exactly and SHALL NOT be inferred from
payload, trace, timestamp, arrival position, status or a binding-requirement
identity. The binding relationships above are not identity components: reusing
one complete key with different relationship facts is an identity contradiction
and refuses. Re-observing the same complete key with the same facts is replay of
one subject. A second delivery identity related to one send is a distinct
delivery permitted only by the channel cardinality. A second effect identity
validly bound to the same singleton effect occurrence and establishing attempt
is retained as a distinct duplicate-business-effect outcome. Relabeling a
delivery as an effect, or binding an effect to the wrong attempt, is instead a
binding refusal.

A receipt is an observation/provenance record about a delivery, not another
name for the delivery. Multiple receipt records may replay one delivery key;
they do not mint another delivery or effect. The declared tuples have a
mechanical lexicographic key order for stable comparison only. Delivery order,
attempt retry order and effect causality arise only from authored protocol,
channel or retry relations, never from key order, timestamps or arrival order.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-051-AC-1 | One message sent once, delivered twice and applied as one payment effect retains cardinalities 1, 2 and 1 for the respective subjects. | Test (TC-051) |
| FR-051-AC-2 | Relabeling a delivery as an effect without an exact effect identity and establishing-attempt relationship refuses, while a second exact effect identity validly bound to the same singleton effect occurrence yields the authored duplicate-effect violation. | Test (TC-051) |
| FR-051-AC-3 | A payment attempt with no established effect remains an attempt and cannot activate effect-dependent compensation. | Test (TC-051) |
| FR-051-AC-4 | A transport trace ID substituted for an authored workflow, attempt or effect identity refuses before conformance evaluation. | Test (TC-051) |
| FR-051-AC-5 | Two deliveries of one send, two attempts at one node and two effects of one attempt remain distinct exactly when their complete authority-qualified identity keys differ; changing only payload, trace, timestamp, arrival position or display text never creates a subject. | Test (TC-051) |
| FR-051-AC-6 | Re-observing one complete effect key is replay and creates no new effect, while a second exact effect identity for the same node and establishing attempt remains a distinct duplicate-business-effect outcome; a delivery merely relabeled as an effect refuses binding. | Test (TC-051) |
| FR-051-AC-7 | Receipt identity and delivery identity remain different kinds, and sorting delivery keys establishes neither FIFO order nor a causal edge. | Test (TC-051) |
| FR-051-AC-8 | Reusing a delivery, attempt or effect key with a different send, node, role, operation or establishing-attempt relationship refuses as an identity contradiction rather than minting another subject. | Test (TC-051) |

## Dependencies

- [FR-050](FR-050-bind-protocol-instances.md).
- [Shared ecosystem binding](../foundation/FR-035-bind-ecosystem-subjects.md).
- F-owned observation/correlation contract under OB01.
- [FR-287](../../objects/protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  defines the common runtime key and non-causal order.
