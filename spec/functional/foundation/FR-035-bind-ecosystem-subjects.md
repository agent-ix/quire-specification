---
id: FR-035
title: "Retain explicit ecosystem scope identities in assessment requests"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
  - target: ix://agent-ix/quire-specification/FR-210
    type: references
  - target: ix://agent-ix/quire-specification/FR-212
    type: references
  - target: ix://agent-ix/quire-specification/FR-213
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
## Description

When binding an ecosystem assessment request, the binder SHALL retain the declared workflow, role, relationship and population-scope identities needed by that request and refuse identity substitutions that change its subject.

## Inputs

An admitted clause, declaration keys from the selected domain package, configuration identities, B-owned protocol instance bindings, F-owned observation bindings and explicit finite snapshot or window scope references.

## Outputs

A request bound to the exact declared subjects and completeness dependencies, or a located identity/correlation refusal or explicitly incomplete binding result.

## Behavior

A trace ID, timestamp, endpoint or matching textual name alone does not establish a business relationship. Workflow instances, role instances, message deliveries, attempts and effects remain distinct subjects. Scope references preserve the selected snapshot/window, membership declaration and observation/progress dependencies; static package identity remains separate. Unknown required membership cannot establish aggregate success. B/F define the protocol and observation binding contracts; this requirement introduces no model store, correlation heuristic or alternate schema.

The [package contract](../../../proposals/quire-v1/package-contract.md) defines the
typed role mapping. Configuration and binding contracts must agree with the
linked requirements. Concrete runtime bindings are separately identified inputs;
they cannot override static selections. Required role mappings and records needed
by a currently activated obligation are distinct prerequisites.

Runtime binding checks the package contract's stage-specific prerequisites
against the already linked selections. A prior successful type-check cannot
supply missing required runtime scope or authority. The domain package
identity and native raw-byte digests remain separate; a runtime input
cannot override the linked domain package or profile.

Every concrete runtime subject SHALL be identified by one authority-qualified
opaque reference consisting of the exact selected binding authority identity,
the subject kind given as its effective declaration key (domain package
identity, IR node identity), and the exact non-empty
object-identity bytes supplied by the runtime input. Equality and hashing are
componentwise over that tuple without parsing, trimming, normalization or
reconstruction. The binding requirement names the typed slot and authority; it
is not the runtime object identity that fills the slot.

Relationships from the runtime subject to a static declaration, workflow, role,
channel, node, attempt or configuration are validated separately and do not
become additional identity components. Reusing one complete identity key with
incompatible kind, authority or relationship facts is an identity contradiction
and SHALL refuse the affected binding. Missing or empty components SHALL refuse
rather than default. Lexicographic key order is used only for deterministic
collection comparison and SHALL NOT establish source, causal, delivery, retry or
arrival order.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-035-AC-1 | Two concurrent orders with shared infrastructure retain distinct request subjects and explicitly related payment/shipment/refund identities. | Test (TC-035) |
| FR-035-AC-2 | A delivery ID or telemetry trace ID substituted for the required effect or workflow identity refuses binding. | Test (TC-035) |
| FR-035-AC-3 | A population request retains the exact snapshot/window and membership/completeness dependencies; changing that scope changes the runtime request subject. | Test (TC-035) |
| FR-035-AC-4 | Unknown required membership yields an explicitly incomplete assessment and cannot establish a successful aggregate. | Test (TC-035) |
| FR-035-AC-5 | A missing, repeated or incompatible required role mapping refuses the dependent request; an explicit compatible binding shared by two distinct roles is admitted. | Test (TC-035) |
| FR-035-AC-6 | A runtime configuration selecting another model/profile/binding-contract meaning refuses against the existing linked subject rather than overriding it. | Test (TC-035) |
| FR-035-AC-7 | Changing only the runtime resource limit changes the retained configuration and assessment input identity while preserving the static subject. | Test (TC-035) |
| FR-035-AC-8 | A valid mapping for an unactivated compensation does not require a recovery record; once required, an unavailable observation remains incomplete rather than becoming optional-none or false. | Test (TC-035) |
| FR-035-AC-9 | A previously linked template still refuses a missing required runtime scope selection and reports unavailable required observations as incomplete; a runtime input naming another domain package or profile cannot override its linked selection. | Test (TC-035) |
| FR-035-AC-10 | Equal object-identity bytes under a different authority identity or subject declaration key produce distinct canonical keys; offering one where the other authority/kind is required refuses rather than returning semantic object equality, while display, trace, timestamp, payload and source-locus changes alone do not mint a runtime subject. | Test (TC-035) |
| FR-035-AC-11 | A missing or empty identity component, or one complete identity key rebound to incompatible typed relationship facts, refuses without a generated/default identity or dependent result. | Test (TC-035) |
| FR-035-AC-12 | Permuting runtime subjects preserves their canonical lexicographic key order and changes no authored causal, delivery or retry relation. | Test (TC-035) |

## Dependencies

- [Shared drafting foundation](../../../proposals/quire-v1/shared-foundation.md).
- [Composed architecture](../../assurance/AD-001-composed-native-language.md).
- [FR-287](../../objects/protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  extracts the common runtime identity value and comparison rule.

Proposed requirement for the composed profile; it does not amend historical
accepted definitions or establish an implemented capability.
