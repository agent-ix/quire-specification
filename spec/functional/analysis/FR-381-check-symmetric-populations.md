---
id: FR-381
title: "Check symmetric population declarations for identity transparency"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-011
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-382
    type: references
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
---

# FR-381: Check symmetric population declarations for identity transparency

## Description

When a population is declared `symmetric`, the checker SHALL refuse, at
authoring time, every clause form that observes the identity of the
population's objects other than by equality, dereference, membership or
passing as a value.

## Inputs

A domain package whose population declaration carries `symmetric`, and the
checked units that read that population.

## Outputs

A checked package carrying the annotation, or a located refusal naming the
breaking node and the population.

## Behavior

**The annotation.** A population declaration in a domain package may carry
`symmetric`, stating that its objects' identities are interchangeable in
every clause that reads it. The population artifact declares it as
`symmetric: true`; the model contract's population record, and the semantic
IR population record lifted from it, carry it as the Boolean member
`symmetric`, `false` when the artifact does not declare it
([FR-011](../foundation/FR-011-validate-populations.md)).
The annotation enters the checked package with the population's declaration,
so it enters the `package_id` and every obligation identity over the
package. The annotation applies no reduction; a request opts in under
[FR-382](FR-382-select-state-space-reductions.md).

**Identity-transparent reads.** A clause reads a symmetric population's
objects transparently when it observes their identity only by reference
equality and inequality, by dereference (field read, `deref`, `reaches`), by
membership, and by passing a reference as a value.

**Breaking forms.** These forms observe identity and are refused when the
reference they apply to is, or contains, a reference into a symmetric
population:

- an order over references (`<`, `<=`, `>`, `>=`), already ill-typed;
- an order-sensitive traversal of a `Set` or `Bag` whose element type
  contains such a reference, such as `fold` or `reduce`, because the
  traversal follows the identity-sorted encoding of
  [FR-181](../runtime/FR-181-simulate-finite-executions.md);
- a conversion of such a reference to a value of another type, including
  text;
- a reference literal naming a key of the population.

`count`, `sum`, `exists`, `forall`, `filter`, `map` into a set or bag,
`contains` and `size` are order-independent and stay transparent.

**Scope of the check.** The checker refuses a breaking form in every clause
of every unit that reads an annotated population: operation preconditions
and postconditions, invariants, temporal clause atoms, `terminal when`
predicates, state constraints ([FR-382](FR-382-select-state-space-reductions.md))
and refinement rows ([FR-375](FR-375-declare-model-refinement.md)). The
refusal is `ill_typed`/`identity-observing-form` at the breaking node,
naming the form and the population. The same forms over an unannotated population are
admitted.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-381-AC-1 | A population declared `symmetric` carries the annotation into the checked package, and adding or removing it changes the `package_id`. | Test (TC-320) |
| FR-381-AC-2 | A `fold` over a `Set<Reference<T>>` field whose references point into a symmetric population refuses `ill_typed`/`identity-observing-form` at the `fold` node, naming the population; the same unit without the annotation is admitted. | Test (TC-320) |
| FR-381-AC-3 | A reference literal naming a key of a symmetric population, and a conversion of such a reference to text, each refuse `ill_typed`/`identity-observing-form` at their node, naming the population, in a precondition, a postcondition, an invariant, a temporal atom, a `terminal when` predicate, a state constraint and a refinement row. | Test (TC-320) |
| FR-381-AC-4 | Reference equality, field read, `deref`, `reaches`, membership, `count`, `exists`, `forall` and `size` over a symmetric population are admitted. | Test (TC-320) |

## Dependencies

- [FR-011](../foundation/FR-011-validate-populations.md) owns population
  declarations.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  identity-sorted encoding of set and bag contents.
- [FR-382](FR-382-select-state-space-reductions.md) owns the request opt-in
  that the annotation permits.
- [FR-375](FR-375-declare-model-refinement.md) owns refinement rows.
- [FR-272](../../objects/foundation/FR-272-typed-diagnostic-cause.md) types
  the `ill_typed`/`identity-observing-form` cause.

## References

- Design: quire-spec-language ADR-021 (SYM-3, SYM-8, QS-3, RU-5).
- Tickets: Linear STD-134; paired QSL-368.
