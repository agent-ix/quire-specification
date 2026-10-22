---
id: FR-381
title: "Check symmetric population declarations for identity transparency"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-011
    type: references
  - target: ix://agent-ix/quire-specification/FR-145
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
- a `fold` or `reduce` over a `Set` or `Bag` whose element type contains
  such a reference and whose step is outside
  [FR-145](../expressions/FR-145-convert-and-query-collections.md)'s
  commutative and associative catalog, because such a traversal follows the
  identity-sorted encoding of
  [FR-181](../runtime/FR-181-simulate-finite-executions.md). The class is
  decided by catalog membership alone: a step outside the catalog is a
  breaking form even when its result happens not to depend on order, because
  FR-145 admits no other proof of order independence. A `fold` or `reduce`
  whose step is in the catalog is not a breaking form and is checked as
  FR-145 checks it;
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
naming the form and the population. Over a population that is not annotated,
each form is checked by the rules that apply without the annotation, and no
form is refused as `identity-observing-form`.

**Precedence over FR-145.** A `fold` or `reduce` over a `Set` or `Bag` whose
step is outside FR-145's catalog is refused in every unit, so it is reported
once, with one cause decided per fold from the fold's source type: when the
source's static type contains a reference into an annotated population, the
refusal is `ill_typed`/`identity-observing-form` naming that population;
otherwise it is FR-145's `ill_typed`/`operator-ineligible`. A unit that reads
an annotated and an unannotated population therefore reports each fold by
its own source. The identity-observing cause takes precedence because it
names the population the author can unannotate or restructure.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-381-AC-1 | A population declared `symmetric` carries the annotation into the checked package, and adding or removing it changes the `package_id`. | Test (TC-320) |
| FR-381-AC-2 | A `fold` over a `Set<Reference<T>>` field whose references point into a symmetric population, with a step that chooses an element (outside FR-145's catalog), refuses `ill_typed`/`identity-observing-form` at the `fold` node, naming the population; the same unit without the annotation refuses `ill_typed`/`operator-ineligible` at the `fold`; and `fold<Tally>(acc, p in self.peers: acc + 1, identity: 0)`, a catalog step, with `type Tally = Integer;`, is admitted in both units. | Test (TC-320) |
| FR-381-AC-3 | A reference literal naming a key of a symmetric population, and a conversion of such a reference to text, each refuse `ill_typed`/`identity-observing-form` at their node, naming the population, in a precondition, a postcondition, an invariant, a temporal atom, a `terminal when` predicate, a state constraint and a refinement row. | Test (TC-320) |
| FR-381-AC-4 | Reference equality, field read, `deref`, `reaches`, membership, `count`, `exists`, `forall` and `size` over a symmetric population are admitted. | Test (TC-320) |
| FR-381-AC-5 | In one unit that reads the symmetric population `config_history` and a second population `drafts` that is not annotated, the same element-choosing `fold` over a `Set<Reference<ConfigVersion>>` field refuses `ill_typed`/`identity-observing-form` naming `config_history`, and over a `Set<Reference<Draft>>` field refuses `ill_typed`/`operator-ineligible`, each at its own `fold` node. | Test (TC-320) |

## Dependencies

- [FR-011](../foundation/FR-011-validate-populations.md) owns population
  declarations.
- [FR-145](../expressions/FR-145-convert-and-query-collections.md) owns the
  set-and-bag step catalog and the `operator-ineligible` refusal outside it.
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
