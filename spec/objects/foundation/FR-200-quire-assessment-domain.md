---
id: FR-200
title: "Quire semantic assessment bounded context"
type: FR
object: domain
relationships:
  - target: ix://agent-ix/quire-specification/FR-201
    type: contains
---
# FR-200: Quire semantic assessment bounded context

## Description

When a requirement references a domain noun, the specification reader SHALL resolve that noun through the object artifacts contained in this bounded context rather than through a restatement in an individual behavioral requirement.

## Bounded Context

This context owns the **meaning of the nouns that the native Quire language
assesses**: the identities compared during assessment, the vocabularies matched
by exact string, and the rules by which two such things are judged equal or
ordered. It is the system of record for identity, equality, key order and
absence semantics across the state, protocol/choreography, temporal and
observation families.

It does **not** own:

- **Domain declarations and their producer semantics.** The shared model
  producer owns model, configuration and relationship identity; this context
  consumes them by identity and never declares a parallel model schema.
- **Evidence.** Quoin retains evidence authority; this context creates no
  evidence store or verification-method registry.
- **Implementation.** Nothing here selects a backend, a wire layout or a
  compiler stage.
- **Temporal representation.** `tl-*` syntax and evaluators are consumed behind
  a versioned native-Quire lowering boundary; they are not a second authored
  source language.

Object artifacts in this context are **structural**. Each one extracts what the
behavioral requirements already state and says it once. An object artifact
introduces no obligation of its own: where the prose does not establish a key or
an order, the object artifact records that the prose does not establish it,
rather than supplying one.

## Identity discipline

Every object artifact contained in this context SHALL declare, explicitly:

- **Identity kind** — one of: *authored*, an identity written in native source;
  *derived*, an identity computed structurally from the object's own components;
  or *value*, no identity at all, compared componentwise.
- **The key** — which components constitute identity. Equality is componentwise
  over the key and nothing else. Display name, source path, span, message text,
  locale and rendering never participate in identity
  ([FR-009](../../functional/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/FR-047-emit-typed-located-causes.md)).
- **Key order** — a total order over the key, used only to compare and diff
  collections. A key order carries no causal meaning.
- **Causal order** — where the object participates in a causal structure, the
  authored partial order of
  [FR-052](../../functional/FR-052-represent-bounded-control.md). It is never
  derived from key order, timestamp or source position.
- **Absence** — an absent key component compares only to absence. It never
  compares to a default, an empty value or an implicit option-none.
- **Digest domain** — where a key contains a digest, the domain that digest was
  derived under ([FR-201](FR-201-canonical-identity-domain.md)). An identity
  crossing domains is unequal regardless of byte equality.

## Ubiquitous Language

- **Authored** — written by a person in native Quire source, and therefore
  carrying a source identity, revision and span.
- **Derived** — computed from other identified things under a named rule, and
  changing whenever any input to that rule changes.
- **Key** — the exact component tuple that decides equality.
- **Key order** — the total order over keys used for collection comparison.
- **Causal order** — the authored partial order over protocol nodes.
- **Domain** — the named algorithm and selection under which a digest identity
  is derived.
- **Absence** — the explicit state of a component that was not supplied, held
  distinct from every supplied value.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-200-AC-1 | Every object artifact contained in this context declares an identity kind, a key, an absence rule, and either a key order or an explicit statement that the prose establishes none. | Inspection |
| FR-200-AC-2 | No object artifact contained in this context introduces an obligation absent from the behavioral requirement it extracts. | Analysis |
| FR-200-AC-3 | A key order declared by a contained object carries no causal meaning and is never cited as evidence of a causal edge. | Analysis |

## Dependencies

- [FR-009](../../functional/FR-009-preserve-object-identity.md) supplies the
  universe/type/object identity triple and the rule that structural record
  equality never replaces object identity.
- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md) supplies
  the cross-family vocabulary and its ownership.
- [Functional capability ledger](../../../docs/functional-capability-ledger.md)
  records the workstream that owns each family.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
