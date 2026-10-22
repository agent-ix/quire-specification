---
id: FR-382
title: "Select and admit state-space reductions for a model-check item"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-381
    type: references
  - target: ix://agent-ix/quire-specification/FR-383
    type: references
  - target: ix://agent-ix/quire-specification/FR-385
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
---

# FR-382: Select and admit state-space reductions for a model-check item

## Description

When a model-check item selects symmetry, partial-order reduction or a state
constraint, the request SHALL carry each selection as a typed member, and the
engine SHALL admit every selected reduction against the checked package, the
subject and the claim before any expansion, never dropping, adding or
replacing one.

## Inputs

An [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
request item over a model subject, its checked package, its universes and
initial states, and its claim.

## Outputs

An admitted reduction selection for [FR-383](FR-383-explore-under-state-space-reduction.md),
or a settled item ([FR-385](FR-385-report-reduced-verdicts.md)) or a refused
request.

## Constraint clause syntax

A state constraint is a state clause:

```ebnf
state-clause = … | 'constraint', ident, 'using', ident, 'on', type-name,
               'at', 'current', block ;
```

It is checked like an invariant body: a `Boolean` expression over the
current state with `self` ranging over the objects of the named type, and it
holds at a state when it holds for every object of that type.

## Behavior

**Request members.** A model-check request item carries `reductions`, with
three members:

- `symmetry` — a list of `SymmetryDeclaration{population, classes}`, at most
  one per population, sorted by population key: `population` names a
  universe of the request, and `classes` is a list of key lists, each class
  sorted by key bytes and the classes sorted by their first key. A key in no
  class, or in a class of one, is fixed.
- `partial_order` — absent, or `{"proviso": "breadth-first-revisit"}`.
- `constraint` — absent, or the node key of one checked `constraint` clause.

**Identity.** The symmetry declarations and the partial-order selection enter
the item's obligation identity beside the subject; a reduced and an unreduced
request for one claim are different obligations, and a verdict on one never
settles the other. The state constraint is a method parameter of the engine,
like the caller's limits: it is recorded in the item's terminal record, stays
out of the obligation identity, and is part of the request identity, so a
result is never reused across requests with different constraints. A
constraint never changes what a verdict means, except that a constraint that
evaluates undefined at a reachable state refutes the item
([FR-383](FR-383-explore-under-state-space-reduction.md)).

**Well-formed symmetry declaration.** A declaration whose population is not
declared `symmetric` ([FR-381](FR-381-check-symmetric-populations.md)), is
not a universe of the request, names a key outside that universe, or puts a
key in two classes refuses the request `invalid_runtime_input`/
`invalid-value`, naming the declaration.

**Symmetry group.** The group `G` is the product, over every declaration, of
the symmetric group of each class. A permutation in `G` acts on a state by
renaming every reference whose universe and key it moves, in every field,
collection and record, and re-encoding the state key
([FR-181](../runtime/FR-181-simulate-finite-executions.md)); it acts on a
transition identity by renaming its receiver and reference arguments.

**Symmetry admission.** For every generator of `G` (per class of `m` keys in
key order, the transposition of its first two keys and, for `m >= 3`, the
cycle through all `m`), applying the generator to each initial state yields a
state whose key equals some initial state's key. A finite set closed under
the generators is closed under `G`. Under admission, and with the
authoring-time check of FR-381, every permutation in `G` maps transitions to
transitions, fixes the initial states as a set and preserves every atom's
truth. A declaration whose initial states fail the check settles the item
`inconclusive`, cause `symmetry-broken` naming the population, the initial
state and the generator ([FR-385](FR-385-report-reduced-verdicts.md)).

**Partial-order selection.** A selection over a transition system that
supplies no footprints ([FR-383](FR-383-explore-under-state-space-reduction.md))
settles the item `unsupported`, cause
`unsupported_projection`/`unsupported-requested-capability`, naming
partial-order reduction.

**Pre-check.** Before the first expansion the engine looks up each selected
reduction's row in the preservation table (FR-385) for the claim's form and
fairness set, admits each symmetry declaration, instantiates footprints, and
resolves the constraint clause. Any failure settles the item before
expansion.

**Advertisement and negotiation.** A backend's manifest entry advertises the
reductions it applies, as a subset of `symmetry`, `partial-order` and
`constraint`. Candidates are computed by kind as
[FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) states; the
single candidate's negotiation arm settles an item that selects a reduction
its manifest does not advertise `unsupported`, cause
`unsupported_projection`/`unsupported-requested-capability`, naming the
reduction. Negotiation never removes, adds or replaces a selected reduction,
and the engine never answers a reduced request by running unreduced, or an
unreduced request by running reduced.

**Dense-time subjects.** These rules apply to a subject that declares
`time dense`; a `time tick` subject is read through digital time
([FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md)) and keeps the
rules above unchanged.

- A symmetry permutation SHALL permute each object of a declared class
  together with its clock fields: the zone's clocks of object `k` move to the
  clocks of the object `k` is mapped to.
- The symbolic provider SHALL canonicalise a symbolic state by the
  permutation that canonicalises its discrete state, ties between objects
  with equal discrete state broken by the least permutation in key order, and
  SHALL apply the same permutation to the zone. Zone coverage is tested on the
  canonicalised state. A state and its image under a permutation
  canonicalise to equal states when the discrete permutation is unique (no
  two objects of a class have equal discrete state); otherwise they may stay
  distinct, which is sound and only reduces less.
- The symbolic provider's manifest entry SHALL advertise `symmetry` and no
  other reduction, so negotiation SHALL settle an item over a `time dense`
  subject that selects partial-order reduction or a state constraint
  `unsupported`, `unsupported_projection`/`unsupported-requested-capability`,
  naming the first such reduction in the order partial-order, constraint,
  with no candidate run and no state expanded.
- The symmetry declarations enter the obligation identity as for an untimed
  subject.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-382-AC-1 | A request item carries `reductions` with `symmetry`, `partial_order` and `constraint` in the stated canonical forms; a non-canonical order of declarations, classes or keys refuses `invalid_runtime_input`/`invalid-value`. | Test (TC-321, TC-327) |
| FR-382-AC-2 | Two requests for one claim that differ only in a symmetry declaration or the partial-order selection have different obligation identities; two that differ only in the constraint have the same obligation identity, different request identities, and record different constraints in their terminal records. | Test (TC-327) |
| FR-382-AC-3 | A symmetry declaration on an unannotated population, on a population that is not a universe of the request, naming a key outside the universe, or putting a key in two classes refuses the request `invalid_runtime_input`/`invalid-value`, naming the declaration. | Test (TC-320, TC-321) |
| FR-382-AC-4 | An initial state that some generator of `G` maps to no initial state settles the item `inconclusive`, cause `symmetry-broken`, naming the population, the initial state and the generator, before any expansion; the declaration `[[a], [b, c]]` over the same initial state is admitted. | Test (TC-321) |
| FR-382-AC-5 | A `constraint` clause is checked like an invariant body and holds at a state when it holds for every object of its type; a non-Boolean body refuses `ill_typed`/`non-boolean-root`. | Test (TC-324) |
| FR-382-AC-6 | An item selecting a reduction its single candidate does not advertise, or partial-order reduction over a transition system with no footprints, settles `unsupported` with `unsupported_projection`/`unsupported-requested-capability` naming the reduction, and no unreduced run happens. | Test (TC-327) |
| FR-382-AC-7 | A `time dense` model with population `Timer` over universe `{t1, t2}` (each with `armed: Bool` and clock `x`; `arm` with precondition `not self.armed`, guard `x >= 1`, reset `x`; `disarm` with precondition `self.armed`, guard `x >= 1`, reset `x`): with symmetry `[[t1, t2]]` its deadlock-freedom item settles `proved` as with no reduction and stores fewer symbolic states, and `always holds(not (t1.armed and t2.armed))` settles `refuted` as with no reduction, its counterexample replaying; the symbolic state with `t1` armed, `t2` unarmed and zone `Z`, and its image with `t1` unarmed, `t2` armed and `Z` with the clocks of `t1` and `t2` swapped, canonicalise to equal states; the deadlock-freedom item selecting partial-order reduction, and the same item selecting a state constraint, each settle `unsupported`, `unsupported_projection`/`unsupported-requested-capability` at negotiation naming the reduction, with no state expanded; the same item selecting both partial-order reduction and a state constraint settles the same way naming partial-order reduction only. | Test (TC-963) |

## Dependencies

- [FR-381](FR-381-check-symmetric-populations.md) owns the `symmetric`
  annotation and the authoring-time half of symmetry admission.
- [FR-383](FR-383-explore-under-state-space-reduction.md) owns exploration
  under an admitted selection, and footprints.
- [FR-385](FR-385-report-reduced-verdicts.md) owns the preservation table and
  the verdicts this requirement settles.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the request members and manifest advertisement.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns
  candidate computation.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns state keys.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns
  the state-clause production this requirement extends.

## References

- Design: quire-spec-language ADR-021 (RD-1 to RD-3, SYM-1, SYM-2, SYM-4,
  SYM-5, SC-1, SC-3, EI-1, RV-6, RV-8, RV-9, QS-1, QS-3, QS-9); ADR-026 §15
  and QSL FR-430 (reductions over a dense-time subject).
- Tickets: Linear STD-134; paired QSL-368.
