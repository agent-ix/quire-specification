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
---
# FR-213: Workflow instance

## Description

When a request names a workflow instance, the binder SHALL treat it as an authored subject distinct from its static protocol definition and from every other instance, refusing a trace identifier, endpoint, timestamp, provider or matching textual name supplied in its place.

## Properties

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
- **Key** — **the prose does not fix the key components.** It fixes only what
  is excluded from the key: a transport trace ID (FR-051-AC-4, FR-035-AC-2), a
  delivery ID (FR-035-AC-2), a timestamp, an endpoint, a matching textual name
  (FR-035) and a shared provider or trace infrastructure (FR-050-AC-3,
  FR-111-AC-1). Equality is componentwise over the key and nothing else;
  display name, source path, span, message text, locale and rendering never
  participate (FR-035, FR-049-AC-7). This artifact supplies no key components.
- **Key order** — none. Because the key components are not fixed, no total
  order over the key is stated and none is supplied here. Ordering two workflow
  instances by start timestamp or by source position would contradict FR-052,
  which forbids deriving order from source order or timestamps.
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
- **Digest domain** — no component named by the prose is a digest. Where a
  workflow instance identity is carried beside a digest in provenance, that
  digest's domain is selected under
  `ix://agent-ix/quire-specification/FR-201`, and an identity crossing domains
  is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-213-AC-1 | Two concurrent workflow instances that share a provider or trace infrastructure compare unequal. | Analysis |
| FR-213-AC-2 | A trace ID, delivery ID, timestamp, endpoint or matching textual name supplied for a workflow instance identity does not resolve. | Analysis |
| FR-213-AC-3 | A workflow instance and its static protocol definition are not equal. | Analysis |
| FR-213-AC-4 | This artifact declares no key components and no key order for a workflow instance, and cites the prose exclusions instead. | Inspection |

## Dependencies

- [FR-050](../../functional/FR-050-bind-protocol-instances.md) binds workflow
  instances and keeps them distinct from static definitions.
- [FR-051](../../functional/FR-051-preserve-communication-identities.md)
  establishes the authored workflow identity and refuses trace-ID substitution.
- [FR-035](../../functional/FR-035-bind-ecosystem-subjects.md) keeps workflow
  instances distinct subjects and governs scope and completeness dependencies.
- [FR-111](../../functional/FR-111-correlate-related-instances-explicitly.md)
  admits only explicit typed relationships between related instances.
- [FR-052](../../functional/FR-052-represent-bounded-control.md) supplies the
  authored causal order.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

No requirement states what constitutes a workflow instance's identity; it states
only what cannot stand in for it.

1. Whether the key includes the protocol definition, language edition, profile
   or package identity is unstated.
2. Whether the key includes the configuration identity that FR-050 requires the
   instance to be bound to is unstated.
3. No key order over workflow instances can be established until the key
   components are.
