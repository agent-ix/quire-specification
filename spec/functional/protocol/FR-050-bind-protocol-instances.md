---
id: FR-050
title: "Bind protocol roles and instances to exact subjects"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-035
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-210
    type: references
  - target: ix://agent-ix/quire-specification/FR-211
    type: references
  - target: ix://agent-ix/quire-specification/FR-212
    type: references
  - target: ix://agent-ix/quire-specification/FR-213
    type: references
  - target: ix://agent-ix/quire-specification/FR-214
    type: references
  - target: ix://agent-ix/quire-specification/FR-215
    type: references
  - target: ix://agent-ix/quire-specification/FR-217
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
## Description

When admitting a native protocol definition, the protocol linker SHALL bind every role, role instance, workflow instance, message and channel reference to its exact typed declaration and configuration identity before conformance evaluation.

## Inputs

An admitted native package, protocol definition, A-owned source/profile identities,
D-owned model/configuration identities and explicit workflow-instance bindings.

## Outputs

A finite linked protocol representation with exact subject references, or a
located typed refusal for every unresolved, ambiguous or substituted subject.

## Behavior

The linker SHALL keep roles distinct from components, endpoints and channels.
The linker SHALL keep role instances and workflow instances distinct from their
static definitions. The linker SHALL refuse duplicate role authorities,
ambiguous instance bindings and name-only substitutions. Internal and external
participants use the same identity rules; an external role is not assumed to be
fully observable.

The linker and runtime binder SHALL use these exact identity keys, where the
qualified protocol-definition identity is the FR-288 value rather than a
declaration name or unqualified table index:

- a static role is keyed by the qualified protocol-definition identity and its
  exact declaration-local role handle;
- a static channel is keyed by the qualified protocol-definition identity and
  its exact declaration-local channel handle;
- a workflow instance is keyed by one FR-035 authority-qualified opaque runtime
  reference of kind `workflow-instance`;
- a role instance is keyed by one FR-035 authority-qualified opaque runtime
  reference of kind `role-instance`; and
- a message is keyed by one FR-035 authority-qualified opaque runtime reference
  of kind `message`.

A binding-requirement identity identifies the slot to be filled; it is not the
concrete runtime identity that fills the slot. A workflow-instance key SHALL
bind exactly one selected protocol definition and workflow requirement. A
role-instance key SHALL bind exactly one workflow instance, static role slot and
configuration binding. A message key SHALL bind exactly one authored message
obligation and channel context. These relationships do not participate in the
opaque subject key. Reusing one role-instance key for distinct role slots or
workflow instances, or reusing any key with conflicting binding facts, is an
identity contradiction and SHALL refuse. The binder SHALL admit two static
roles that share one compatible participant/provider binding while retaining
distinct static roles and role instances.

Missing, empty, wrong-kind, foreign-authority or colliding offered identity
components SHALL refuse the affected binding rather than being generated,
normalized or replaced by a table position, source span, trace identifier,
timestamp, display name or provider identity.

Each declared tuple has the mechanical lexicographic key order required by
FR-200. That order exists only for stable comparison and diffing and SHALL NOT
establish source order, role precedence, message order or causality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-050-AC-1 | A protocol with internal order, inventory and fulfillment roles plus an external payment role retains every definition and instance identity independently. | Test (TC-050) |
| FR-050-AC-2 | Substituting an endpoint or channel identity for a role, or a component identity for a role instance, refuses linking with the affected source location. | Test (TC-050) |
| FR-050-AC-3 | Two concurrent workflow instances sharing one provider remain distinct assessment subjects. | Test (TC-050) |
| FR-050-AC-4 | An unresolved, duplicate or ambiguous role/instance reference prevents dependent evaluation without removing independent valid definitions. | Test (TC-050) |
| FR-050-AC-5 | Equal producer-local identity bytes under different authority or subject-kind components remain unequal; two static role slots remain distinct even when they share display text and one compatible participant/provider binding. | Test (TC-050) |
| FR-050-AC-6 | A binding requirement without one exact non-empty concrete runtime identity, or with a wrong-kind, foreign-authority or colliding identity, refuses that binding; the requirement identity alone never stands in for the concrete identity. | Test (TC-050) |
| FR-050-AC-7 | Reusing one complete workflow-, role-instance- or message-identity key with incompatible definition, role, workflow, configuration, obligation or channel binding facts refuses as an identity contradiction. | Test (TC-050) |
| FR-050-AC-8 | Lexicographically sorting role, role-instance, workflow-instance, channel or message keys changes no authored causal or delivery relation. | Test (TC-050) |

## Dependencies

- [US-006](../../usecase/US-006-assess-distributed-protocols.md).
- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md).
- [Qualified protocol-definition identity](../../objects/protocol/FR-288-qualified-protocol-definition-identity.md).
- [Protocol contract](../../../proposals/quire-v1/protocol-contract.md).
- [FR-287](../../objects/protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  defines the common runtime key and non-causal order.
