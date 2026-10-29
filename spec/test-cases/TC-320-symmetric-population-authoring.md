---
id: TC-320
title: "Symmetric population authoring checks"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-381
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-382
    type: verifies
---

# TC-320: Symmetric population authoring checks

## Description

Admit a `symmetric` population whose clauses read identity transparently,
refuse each identity-observing form at authoring time, and refuse a symmetry
request on an unannotated population.

## Test Procedure

Subject: the ConfigVersion domain package with population `config_history`
of `ConfigVersion` objects (`versionNumber: Int[0, 2]`,
`parent: Option<Reference<ConfigVersion>>`), invariants `ParentOrder` and
`NoCycle` reading `parent` by dereference and equality, and operation
`attemptUpdate` advancing `versionNumber` modulo 3.

1. Declare `config_history` `symmetric`; check the unit; then remove the
   annotation and compare the checked packages.
2. Add a field `peers: Set<Reference<ConfigVersion>>` and a postcondition
   that chooses the new parent by `fold` over `self.peers` (a step outside
   FR-145's catalog); check with and without the annotation. Then replace the
   step by `fold<Tally>(acc, p in self.peers: acc + 1, identity: 0)` with
   `type Tally = Integer;` and check with and without the annotation.
3. With the annotation, add in turn: a reference literal naming key `a` in a
   precondition, a postcondition, an invariant, a temporal atom, a
   `terminal when` predicate, a state constraint and a refinement row; and a
   conversion of `self.parent` to text in an invariant.
4. With the annotation, add clauses using reference equality, field read,
   `deref`, `reaches`, membership, `count`, `exists`, `forall` and `size`
   over `config_history`.
5. Without the annotation, submit a model-check request declaring
   `SymmetryDeclaration{config_history, [[a, b, c]]}`.
6. With the annotation, add a population `drafts` of `Draft` objects that is
   not annotated, a field `others: Set<Reference<Draft>>`, and a second
   postcondition choosing an element by the same `fold` step over
   `self.others`, beside case 2's `fold` over `self.peers`.
7. With the annotation, add a postcondition that folds, with an
   element-choosing step, over `convert<Sequence<Reference<ConfigVersion>>[0,3]>(self.peers)`;
   then one with `convert<OrderedSet<Reference<ConfigVersion>>[0,3]>(self.peers)`;
   then one that binds `convert<Bag<Reference<ConfigVersion>>[0,3]>(self.peers)`
   and does not fold it;
   then one that folds that `Bag` conversion with an element-choosing step;
   then one with `convert<Sequence<Reference<Draft>>[0,3]>(self.others)`.

## Expected Results

Case 1 is admitted with the annotation in the checked package, and the two
checked packages differ (FR-381-AC-1). Case 2 refuses
`ill_typed`/`identity-observing-form` at the `fold` node naming
`config_history`, and without the annotation refuses
`ill_typed`/`operator-ineligible` at the `fold`; the counting `fold` is
admitted in both units (FR-381-AC-2). Each
addition of case 3 refuses `ill_typed`/`identity-observing-form` at its
node naming the population (FR-381-AC-3).
Case 4 is admitted (FR-381-AC-4). Case 5 refuses the request
`invalid_runtime_input`/`invalid-value`, naming the declaration
(FR-382-AC-3). Case 6 refuses `ill_typed`/`identity-observing-form` naming
`config_history` at the `self.peers` fold and `ill_typed`/`operator-ineligible`
at the `self.others` fold (FR-381-AC-5). Case 7 refuses the first two
`ill_typed`/`identity-observing-form` at the `convert` node naming
`config_history`, admits the `Bag` conversion that is not folded and the `Draft`
sequence conversion, and refuses the fold of the `Bag` conversion
`ill_typed`/`identity-observing-form` at the `fold` node naming
`config_history` (FR-381-AC-6).
