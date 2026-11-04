---
id: FR-213
title: "Workflow instance"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-035
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-213: Workflow instance

## Description

When a request names a workflow instance, the binder SHALL treat it as an authored subject distinct from its static protocol definition and from every other instance, refusing a trace identifier, endpoint, timestamp, provider or matching textual name supplied in its place.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| authored_identity | String | 1..1 |  |
| typed_declaration_and_configuration_identity | String | 1..1 |  |
| explicit_binding | String | 1..1 |  |
| distinctness_from_its_definition | String | 1..1 |  |
| distinct_assessment_subject | String | 1..1 |  |
| declared_relationships_to_related_instances | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| key | type not stated in source; see Field sources |
| authored_identity | type not stated in source; multiplicity not stated in source; see Field sources |
| typed_declaration_and_configuration_identity | type not stated in source; see Field sources |
| explicit_binding | type not stated in source; multiplicity not stated in source; see Field sources |
| distinctness_from_its_definition | type not stated in source; see Field sources |
| distinct_assessment_subject | type not stated in source; multiplicity not stated in source; see Field sources |
| declared_relationships_to_related_instances | type not stated in source; multiplicity not stated in source; see Field sources |


| Property | Source | Note |
| --- | --- | --- |
| Authored identity | FR-051-AC-4 | "A transport trace ID substituted for an authored workflow, attempt or effect identity refuses before conformance evaluation." |
| Typed declaration and configuration identity | FR-050 | Bound "to its exact typed declaration and configuration identity before conformance evaluation." |
| Explicit binding | FR-050 | Inputs include "explicit workflow-instance bindings". |
| Distinctness from its definition | FR-050 | "The linker SHALL keep role instances and workflow instances distinct from their static definitions." |
| Distinct assessment subject | FR-050-AC-3, FR-035 | "Two concurrent workflow instances sharing one provider remain distinct assessment subjects." |
| Declared relationships to related instances | FR-035, FR-111 | Related instances are correlated only through "an explicit typed relationship" (FR-111); "A trace ID, timestamp, endpoint or matching textual name alone does not establish a business relationship" (FR-035). |

## Identity and comparison

- **Identity kind** — *authored*. FR-051-AC-4 names "an authored workflow ...
  identity" explicitly, and FR-050 takes explicit workflow-instance bindings as
  an input. It is not derived from any transport fact.
- **Key** — one exact [FR-287](FR-287-authority-qualified-runtime-subject-reference.md)
  of kind `workflow-instance`. Its static protocol definition, workflow binding
  and configuration are separately validated relationships. A provider, trace,
  delivery, timestamp, endpoint or textual name is not a workflow identity.
- **Key order** — FR-287's lexicographic order, used only for canonical
  collection comparison. Start time, source position and provider order are not
  key order and no key order supplies causality.
- **Causal order** — within an instance, the authored partial acyclic order of
  FR-052 governs: "Sequence SHALL add only its authored order"; "Reordering
  equal-time events does not add a causal edge absent from the protocol"
  (FR-052-AC-2). Between two instances, no requirement establishes any causal
  order; relation between instances is only the explicit typed relationship of
  FR-111 and FR-035.
- **Absence** — an absent key component compares only to absence. An
  unresolved, duplicate or ambiguous instance reference "prevents dependent
  evaluation without removing independent valid definitions" (FR-050-AC-4), and
  "Unknown required membership yields an explicitly incomplete assessment and
  cannot establish a successful aggregate" (FR-035-AC-4); absence never becomes
  a default, an empty value or an optional-none (FR-035-AC-8).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-213-AC-1 | Two concurrent workflow instances that share a provider or trace infrastructure compare unequal. | Analysis |
| FR-213-AC-2 | A trace ID, delivery ID, timestamp, endpoint or matching textual name supplied for a workflow instance identity does not resolve. | Analysis |
| FR-213-AC-3 | A workflow instance and its static protocol definition are not equal. | Analysis |
| FR-213-AC-4 | A workflow-instance key compares and sorts only by FR-287; reusing one complete key with a different protocol, workflow binding or configuration refuses as an identity contradiction. | Analysis |

## Dependencies

- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) binds workflow
  instances and keeps them distinct from static definitions.
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  establishes the authored workflow identity and refuses trace-ID substitution.
- [FR-035](../../functional/foundation/FR-035-bind-ecosystem-subjects.md) keeps workflow
  instances distinct subjects and governs scope and completeness dependencies.
- [FR-111](../../functional/observation/FR-111-correlate-related-instances-explicitly.md)
  admits only explicit typed relationships between related instances.
- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) supplies the
  authored causal order.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) defines the
  concrete runtime key and non-causal comparison order.

## Status

Draft. Specification issue #28 fixed the concrete key, relationship boundary
and canonical comparison order. Executable cross-repository evidence remains
planned.
