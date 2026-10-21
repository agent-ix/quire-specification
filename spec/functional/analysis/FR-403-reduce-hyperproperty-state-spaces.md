---
id: FR-403
title: "Reduce hyperproperty state spaces"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-396
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-399
    type: references
  - target: ix://agent-ix/quire-specification/FR-401
    type: references
  - target: ix://agent-ix/quire-specification/FR-381
    type: references
  - target: ix://agent-ix/quire-specification/FR-382
    type: references
  - target: ix://agent-ix/quire-specification/FR-383
    type: references
  - target: ix://agent-ix/quire-specification/FR-385
    type: references
---

# FR-403: Reduce hyperproperty state spaces

## Description

When a hyper or step-relation item is checked under a reduction, the
analyzer SHALL apply that reduction only where the item's preservation row admits it.

## Inputs

The item's form ([FR-396](FR-396-classify-hyperproperty-forms.md)), its
subjects, its fairness sets, and the reductions the request selects.

## Outputs

The reductions applied, named in a reduced proof's basis, or the
`reduction-not-preserving` settlement.

## Preservation rows

A reduction applies exactly where its row reads "yes". Where the row reads
"no", the item settles `inconclusive` with `reduction-not-preserving` before
expansion. Copy-swap symmetry applies only when the analyzer has verified it
on the checked clause.

A "yes" row says the reduction preserves the verdict, not that every provider
builds it. Where a provider builds neither the diagonal action of a key
symmetry nor a state constraint on a hyper product, a request that selects
either for a hyper item whose row reads "yes" settles `unsupported`,
`unsupported_projection`/`unsupported-requested-capability` before expansion,
naming the reduction, and never runs with the selection ignored. When several
reductions are selected, the first whose row reads "no", in the order key
symmetry, partial-order reduction, state constraint, decides; otherwise the
first selected reduction is named.

| Form | Symmetry | Partial-order reduction | State constraint |
| --- | --- | --- | --- |
| HP-1, one subject | yes: the first execution ranges over orbit representatives, the others over full orbits | no: the relation reads every reachable state | refutation decisive; `proved` only when no boundary state is reached |
| HP-2, every variable over one subject, every fairness constraint `whole` | yes, diagonal | no: lockstep pairs positions across traces | lasso decisive; `proved` only when no boundary state is reached |
| HP-2 over two subjects, or with any `each` constraint | no | no | as the row above |
| HP-6, every variable over one subject | yes, diagonal | no: it changes which visible steps align | refutation decisive; `proved` only when no boundary state is reached |
| HP-2 with swappable variables (copy-swap) | yes, for each class whose variables carry no fairness set; a class whose variables carry any fairness set is stored whole | not applicable | as the form's own row |
| HP-3, every variable over one subject | yes, diagonal, with each witness set renamed member by member and re-sorted | no | no: a cut existential state removes a witness |

- **Diagonal symmetry.** One permutation of the admitted symmetry group is
  applied to every component, with the object parameters' stabiliser applied.
  Indexed reads and the match are among the clauses that must be
  identity-transparent. A separate permutation per component would break a
  read that compares one key across traces, and two subjects have unrelated
  universes, so no diagonal exists across subjects.
- **Partial-order reduction.** Reordering independent steps inside one
  component changes which of its steps pairs with another component's step,
  so no lockstep or projected hyperproperty is invariant under it.
- **State constraint on HP-3.** The witness set is an existential over the
  existential subject's behaviours; a cut can empty it, and the resulting
  refutation would not be a refutation of the subject.
- The automaton component and the witness set's automaton states are never
  reduced.

## Copy-swap symmetry

- **Detection.** For every pair of universal variables `a`, `b` of an HP-2 or
  HP-6 clause, the pair is **swappable** when both name the same alias, their
  fairness sets are equal (unmarked read as `whole`), and the swap `σ`, which
  renames `a` to `b` and `b` to `a` in every indexed read and step label,
  leaves `μ_U` and the body unchanged in normal form. The normal form sorts
  the operands of `and`, `or`, `=` and `!=` by their checked encoding,
  flattens nested `and` and `or`, and leaves every other node in place. The
  test is sound and incomplete: a clause symmetric for a deeper reason is not
  detected, and then no swap applies. Swappable variables fall into classes,
  and the **copy group** is the product of the symmetric groups of the
  classes. No request selects it and no author declares it. Detection runs on
  every HP-2 and HP-6 clause and records the copy group; whether the run
  reduces by it is the next three rules.
- **Fairness.** A fairness constraint reads the slot of its variable in the
  stored state, and a quotient moves a concrete copy between slots wherever an
  edge swaps them, so a slot's enabled and taken steps would not be one
  behaviour's and the quotient would decide fair cycles wrongly. A class
  whose variables carry any fairness set, whatever the granularity, is stored
  whole and named in no reduction.
- **Acceptance condition.** The run reduces only if every exchange of the copy
  group fixes the initial state of the body's automaton and maps each of its
  acceptance sets, one for each unbounded `eventually` or `until` that the
  negated body holds, to itself. A swap that exchanges two acceptance sets, as
  the swap of `a` and `b` does for `always holds(v.l @ a = 0) and always
  holds(v.l @ b = 0)`, is not an automorphism of the acceptance condition. The
  rule is all-or-nothing: if the exchange of any class fails, the product is
  stored whole and names no reduction, for the classes that pass as well.
- **Form.** Copy-swap applies to an HP-2 clause only. An HP-6 clause is stored
  whole as its projected product and names no copy-swap, since no exchange of
  the bad-prefix monitor's states is built for the projected product.
- **Canonical state.** The canonical form of a product state is the member of
  its orbit under the copy group with the least key bytes, component keys in
  quantifier order and then the automaton state index. The run's group is the product of the symmetric
  groups of the copy group's classes whose variables carry no fairness set;
  where a provider builds a diagonal key symmetry (see "Preservation rows"),
  that group commutes with it and the run's group is their product.
- **Soundness.** `σ` maps a matched tuple in the domain to a matched tuple in
  the domain with the same body truth, and maps acceptance sets, the rejecting
  monitor state and every fairness constraint onto themselves, so the quotient
  has a fair accepting cycle, or a reachable rejecting state, exactly when the
  product does.
- **Counterexamples.** A counterexample found on the quotient is concretised
  with the copy permutation, repeating a loop that closes on a permuted state
  until the concrete tuple closes, at most the permutation's order. The result
  is an ordinary counterexample in quantifier order. The lasso a refutation
  reports is the canonical lasso of the quotient, concretised, not the
  canonical lasso of the unreduced product: both are real and both replay, but
  they may differ in stem, loop and `trace_position`, and the one reported is a
  function of the request, the same on every run.
- **Verdicts.** A proof under copy-swap names copy-swap with its classes among
  the reductions of its reduced basis. With copy-swap as its only reduction,
  the core's product-closure checker checks its certificate and the proof
  carries the FR-331 certification `certified`. A refutation names no reduction.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-403-AC-1 | Each form admits symmetry, partial-order reduction and a state constraint exactly as its row reads; a reduction a row rejects settles `inconclusive` with `reduction-not-preserving` before expansion. | Test (TC-347) |
| FR-403-AC-2 | On the leaky and secure vault noninterference clauses, copy-swap applies without a request selection and stores 7 and 6 states against 10 and 8 unreduced, the reduced and unreduced runs give the same verdict, and the reduced proof names copy-swap with its class and settles `proved`, certification `certified`. | Test (TC-347) |
| FR-403-AC-3 | A body that is not unchanged under the swap in normal form, or two variables with different fairness sets, admit no copy-swap. | Test (TC-347) |
| FR-403-AC-4 | A counterexample found under copy-swap carries no canonical state, reports the quotient's canonical lasso concretised, byte-equal across repeated runs, and replays as an ordinary counterexample. | Test (TC-347) |
| FR-403-AC-5 | A class whose variables carry any fairness set is stored whole with the unreduced state count and no reduction named; a second class without fairness in the same clause is reduced and named alone. | Test (TC-347) |
| FR-403-AC-6 | A copy group with one class whose swap exchanges two acceptance sets of the body's automaton is stored whole with no reduction named; a clause with one passing class and one failing class is stored whole with none named; a swap that fixes every acceptance set is reduced. | Test (TC-347) |
| FR-403-AC-7 | An HP-6 clause with a swappable pair is stored whole and names no copy-swap. | Test (TC-347) |
| FR-403-AC-8 | Where the provider builds neither, a key symmetry or a state constraint selected over a hyper item whose row reads "yes" settles `unsupported`, `unsupported-requested-capability` before expansion naming the reduction; with a selection whose row reads "no" and another whose row reads "yes", the first "no" in the order key symmetry, partial-order reduction, state constraint decides. | Test (TC-347) |

## Dependencies

- [FR-396](FR-396-classify-hyperproperty-forms.md) states the forms.
- [FR-399](FR-399-settle-hyperproperty-results.md) settles the item.
- [FR-401](FR-401-replay-hyper-counterexamples.md) replays a concretised
  counterexample.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  owns the `certification` a copy-swap proof carries.
- [FR-381](../analysis/FR-381-check-symmetric-populations.md), [FR-382](../analysis/FR-382-select-state-space-reductions.md), [FR-383](../analysis/FR-383-explore-under-state-space-reduction.md) and [FR-385](../analysis/FR-385-report-reduced-verdicts.md)
  own symmetry declarations, reduction selection, partial-order reduction and state constraints, and the preservation table these rows extend.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
