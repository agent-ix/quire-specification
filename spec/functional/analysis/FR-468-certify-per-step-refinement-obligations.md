---
id: FR-468
title: "Query and certify a refinement's per-step simulation obligations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-379
    type: references
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-354
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-485
    type: references
---

# FR-468: Query and certify a refinement's per-step simulation obligations

## Description

When a refinement declaration has no hidden field, no history field and no `any` row, the checker SHALL state its per-step simulation proof as one *initial query* for the initial-state rule and one *step query* for each step row, each a refutation query over the declared types.

The safety half of such a declaration settles `certified` only when every one of these queries and every hypothesis item below carries a verified refutation certificate (FR-354), and the half is the safety half of the refinement item that [FR-379](FR-379-report-and-replay-refinement-counterexamples.md) settles.

## Inputs

An admitted refinement declaration ([FR-375](FR-375-declare-model-refinement.md)) with no hidden field, history field or `any` row, the invariant clauses whose context is an object type of the concrete model, and a proof backend's outcome and refutation certificates for the queries.

## Outputs

For each query, the query itself and a settlement; for the refinement item, the safety half's result with its [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) `certification`.

## Behavior

### The queries

The per-step obligations of [FR-377](FR-377-decide-model-refinement.md) (Engines) are the queries' content; each query is the negation of one obligation, so a refutation of the query proves the obligation. Every query carries the *hypotheses*: the invariant clauses whose context is an object type of the concrete model (the abstract model's invariants are not hypotheses), the same set the per-step simulation record carries (quire-semantic-value FR-123).

- **Initial query.** The negation of: for every concrete initial state satisfying the hypotheses, the state maps to an abstract initial state. It carries the hypotheses so its identity changes with them.
- **Step query**, one per step row, identified by the row's position in the declaration node's `steps` order ([FR-375](FR-375-declare-model-refinement.md)). The negation of: for every pair `(s, s')` of states of the declared types, if the hypotheses hold at `s`, the operation's effective precondition holds at `s`, `(s, s')` is within the operation's frame and its postcondition holds over `(s, s')`, then the row's rule (`stutter`: the mapped state is unchanged by state key; an abstract operation: that transition's precondition, frame and postcondition admit the mapped pair) holds over `(map(s), map(s'))`.

Each query also requires every mapping row, receiver and argument it reads to be defined and determined at `s` and `s'`: a state satisfying the premises at which one is undefined or undetermined is a counterexample to the query.

Each query is one `operation-contract` item ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)) and is settled on its own. Each hypothesis (invariant clause) has its own `operation-contract` item, which carries both its initial-state obligation and its preservation obligation ([FR-485](../ir-proof/FR-485-check-operation-contract-items.md) "Invariant"; an item is identified by its claim and kind, so one invariant clause is one item), and is a *hypothesis item*; they are not part of the refinement's certificate.

### Claim extent

Each query item carries the [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
`extent` that the boundedness architecture (QSL ADR-014 §4) classifies over
the query's roots, as it classifies every other `operation-contract` item.

- A step query's roots are the concrete operation's receiver (`self`),
  result and parameter types; the parameter domain of each `_` argument of
  its abstract operation; the *context population*, the one population of the
  concrete model that covers the receiver's type; each population the
  operation's contract clauses range over (QSL FR-104, check state clauses;
  QSL FR-391, name a domain-package population in a state clause or a holds
  atom), whether by a quantifier, an
  `allInstances` or a `reaches`, because the query reads the clause bodies; and
  each population a `reaches` in the refinement's state mapping walks.
- The initial query's roots are the populations that cover the concrete
  object types the state mapping maps, and each population a `reaches` in the
  state mapping walks, because the query evaluates the mapping at every
  concrete initial state. They are a superset of the covering populations.
- A population of the concrete model that none of these names adds no
  domain. An abstract population adds no domain of its own, since its
  universe is the concrete universe of its source population (FR-377 Inputs).
- A query with no unbounded root is `bounded`. Any other is `unbounded`,
  naming each unbounded domain: a population with no maximum is one
  `population` domain boundable by `cardinality`, and a population a proof
  bound bounds adds none.

When the receiver's type conforms to a member type of two or more
populations of the concrete model, no context population exists: the checker
SHALL refuse `ambiguous_declaration`/`ambiguous-name` at the step row that
names the concrete operation, naming those populations in ascending
declaration-key order, and states no query for the refinement.

A query item is negotiated like every `operation-contract` item
([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)): its
candidates are chosen by kind alone, and its extent meets the candidate's
advertised mode only at the single candidate's arm. An `unbounded` query
item SHALL settle `requires-bound` under a candidate that advertises
`operation-contract` only in `bounded` mode when a finite bound is available,
and is decided only by a candidate that advertises `operation-contract` in
`unbounded` mode.

### Certificate

The certificate of the safety half carries one *base* part, the refutation of the initial query, and one *step* part per step row, in the `steps` order, each the refutation of that row's step query. A part is a solver refutation in a checkable proof format, bound to the query it refutes. The checker SHALL check the certificate as FR-354 does, so a certificate that arrives without its base part, without a row's step part, with a step part in the base position, or with a part whose carried query differs from the core's encoding, settles `inconclusive`, cause `certificate-rejected{rule, at}` with `at` naming the part (FR-354's shape and query mismatch cases). The checker SHALL settle each part by exactly one of the results below, and this list is the one statement of the settle mapping:

- **Verified.** The part's proof refutes its query using checked rules alone. Only a part in this result is `certified`.
- **Unverifiable.** No rule rejects the part and some command of its proof uses a rule outside the checked set (a part the checker does not show wrong). The query settles `proved` with certification `uncertified`, never `certified`.
- **Unencodable.** The core refuses to encode a step row's query (a `_` argument, a protocol row, a row that binds its result, or a term in the row outside the linear-integer fragment the encoding prints; an `any` row has no per-step query at all (FR-468-AC-1), so it is not a part). The row has no expected query, so its part is neither compared nor checked and its query settles `proved` with certification `uncertified`, never `certified`.
- **Rejected.** The checker shows the part wrong (a missing, misplaced or mismatched part or a proof step that does not follow). The half settles `inconclusive`, cause `certificate-rejected{rule, at}`.

A proof that arrives with no certificate at all settles its query `proved` with certification `uncertified`, as an unverifiable part does. A refusal of the core to encode the initial query is not unencodable: the base part then has no expected query to compare, so no carried query equals one and the checker rejects the base part with the rule `query-mismatch` at `base` (a certificate with the wrong number of step parts is still rejected `shape-mismatch` first, as the shape check precedes every query comparison); the half settles `inconclusive`, cause `certificate-rejected{query-mismatch, base}`, never `proved`. Unverifiable and unencodable are never reported as verified, and the half's certification is `certified` only when the base part and every step part are verified.

### Settlement

The checker SHALL settle the safety half by the following rules, taking the first that applies in the order initial query, hypothesis items, then step queries in `steps` order:

- **Initial counterexample.** A backend counterexample to the initial query is a concrete initial state whose image is no abstract initial state: it refutes the refinement ([FR-377](FR-377-decide-model-refinement.md) FR-377-AC-3). The half settles `refuted`, `decisive-counterexample`, refinement failure `InitialNotAbstract`, only after the explicit-state replay of that initial state reproduces it ([FR-379](FR-379-report-and-replay-refinement-counterexamples.md)); a replay that does not reproduce it settles `inconclusive`, cause `replay-parity`.
- **Hypotheses not established.** If any hypothesis item is not `proved`, the half settles `inconclusive`, cause `hypotheses-not-established{hypothesis, result}`, `hypothesis` the invariant clause's node and `result` its item's result. A false invariant makes every step query vacuous, so no step query's refutation may settle the half `proved` while a hypothesis is unproved.
- **Step counterexample.** A backend counterexample to a step query, including to its definedness requirement, is never a refutation, because its pre-state may be unreachable ([FR-379](FR-379-report-and-replay-refinement-counterexamples.md)): the half settles `inconclusive`, cause `induction-not-closed{depth}` with `depth` 1, naming the row. An explicit-state run that reaches the failing state refutes with `undefined-evaluation` or `mapping-undetermined` as [FR-377](FR-377-decide-model-refinement.md) and FR-379 state.
- **Certificate verdicts.** A certificate the checker shows wrong settles the half `inconclusive`, cause `certificate-rejected{rule, at}` (FR-354). Otherwise, with every query and hypothesis item `proved`, the half settles `proved`, basis `decisive-witness`, with `certification` `certified` when every part is verified and every hypothesis item is certified, and `uncertified` when any part is unverifiable or unencodable, or any hypothesis item is not certified (the weakest of the parts and the hypothesis items); an unverifiable or unencodable part is never counted as verified.
- **Other backend results.** A query the backend settles by none of these (a solver `unknown`, a timeout or limit, a missing solver or tool, or a backend fault) takes the result FR-331 records for that outcome (`inconclusive` `solver-inconclusive`, `incomplete` `limit-reached{limit, value, setting}`, `unsupported`, or `failed`), and the half takes the result of the first such query.

When the declaration's `F_A` is non-empty, the liveness half is decided by the explicit-state engine and FR-379's verdict table decides how the two halves combine into the item's one result; this requirement supplies only the safety half's result and certification. A declaration with a hidden field, a history field or an `any` row has no per-step queries and no inductive certificate (FR-377-AC-7); its safety half is decided by the explicit-state engine only.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-468-AC-1 | A refinement of a model with seven step rows and no hidden field, history field or `any` row has one initial query and seven step queries, each its own `operation-contract` item, the step queries in `steps` order; the same declaration with one row changed to `any`, with a hidden abstract field, or with a history field has no per-step query. | Test (TC-950) |
| FR-468-AC-2 | A certificate that arrives without the base part, without one row's step part, with a step part in the base position, or with a carried query differing from the core's settles the half `inconclusive`, `certificate-rejected{rule, at}` naming the part; a proof with no certificate at all settles that query `proved`, `uncertified`. | Test (TC-950) |
| FR-468-AC-3 | With every query and hypothesis item `proved` and every part verified the half settles `proved`, `certified`; with one query's certificate absent or using an unchecked rule it settles `proved`, `uncertified`; one hypothesis item `uncertified` also gives `uncertified`. | Test (TC-950) |
| FR-468-AC-4 | A backend counterexample to one step query, or to a definedness requirement of it, settles the half `inconclusive`, `induction-not-closed{depth: 1}`, naming the row, and never `refuted`; a counterexample to the initial query settles `refuted`, `InitialNotAbstract`, once replay reproduces it, and `inconclusive` `replay-parity` otherwise. | Test (TC-950) |
| FR-468-AC-5 | A concrete invariant clause is a hypothesis of every step query and of the initial query, so the initial query's identity differs with and without the clause, and an invariant of the abstract model is not a hypothesis. | Test (TC-950) |
| FR-468-AC-6 | A concrete model with the invariant `self.value < 0`, whose hypothesis items are not proved, settles the half `inconclusive`, `hypotheses-not-established`, even though every step query is refuted with a verified certificate. | Test (TC-950) |
| FR-468-AC-7 | A query whose backend returns `unknown`, a timeout, a missing solver or a fault takes the FR-331 result for that outcome and the half takes the first such query's result, whatever the other queries returned. | Test (TC-950) |
| FR-468-AC-8 | Over the compare-and-set counter refinement with seven step rows, whose concrete population `counters` has no maximum, each of the eight query items carries extent `unbounded` with one domain, the `counters` population of kind `population`, boundable by `cardinality`, and the abstract population adds none; with a proof bound of 2 on `counters` each is `bounded`. An unbounded population `audits` of another object type that no contract clause ranges over and no state mapping reads adds no domain; an unbounded population that a contract clause of `peek` quantifies over, without a `reaches`, adds its `population` domain to `peek`'s step query alone; an unbounded population that only the state mapping's `reaches` walks adds its `population` domain to the initial query and to every step query. A concrete operation `peek` taking `n: Integer` (no range), and one returning `Integer`, each give `peek`'s step query a second unbounded `integer` domain, the other seven unchanged. With a finite bound available, under a candidate advertising `operation-contract` only in `bounded` mode with `domains` [`population`], each of the eight `unbounded` query items settles `requires-bound`; with no finite bound available, each settles `unsupported`, `unsupported_projection`/`unbounded-extent`; under a bounded-only candidate whose `domains` lack `population`, each settles `unsupported`, `unsupported_projection`/`unsupported-requested-capability` naming `population`. In each `peek` variant, `peek`'s step query under the bounded-only candidate with `domains` [`population`] settles `unsupported-requested-capability` naming `integer`, and with `domains` [`population`, `integer`] and a finite bound available settles `requires-bound`. Under a candidate advertising `unbounded` mode every query item is decided. A second population `spare` of the receiver's type refuses `ambiguous_declaration`/`ambiguous-name` at each step row, naming `counters` and `spare` in ascending declaration-key order, with no query stated. | Test (TC-950) |
| FR-468-AC-9 | The settle mapping of "Certificate": over a refinement with seven step rows, a certificate whose parts are all verified settles every query `proved`, `certified`; one whose base proof uses `hole` (unverifiable), one whose step part for row 2 uses `lia_generic` (unverifiable), and one over a refinement with an eighth row `bump` that has a `_` argument (unencodable, with valid parts for the seven other rows and a present part at `bump`'s position that is neither compared nor checked) each settle the affected query, and so the half, `proved` with certification `uncertified`; none of the three is ever `certified`, a rejected part anywhere in the certificate outranks an unverifiable or unencodable part and settles `inconclusive`, `certificate-rejected{rule, at}`, and a hypothesis item `uncertified` also gives `uncertified`. | Test (TC-950) |
| FR-468-AC-10 | The core refuses to encode the initial query (a mapping row outside the linear-integer fragment): the certificate, whatever its base part carries, is rejected (after the shape check: a wrong number of step parts is rejected `shape-mismatch` first) with the rule `query-mismatch` at `base`, and the half settles `inconclusive`, `certificate-rejected{query-mismatch, base}`, never `proved` and never `uncertified`; a refusal of a step row's query is not this case and is unencodable (FR-468-AC-9); no step part is compared or checked once the base part is rejected. | Test (TC-950) |

## Dependencies

- [FR-377](FR-377-decide-model-refinement.md) owns the obligations the queries negate.
- [FR-379](FR-379-report-and-replay-refinement-counterexamples.md) owns the verdicts, the replay of an initial counterexample and the rule that a per-step failure is never `refuted`.
- [FR-375](FR-375-declare-model-refinement.md) owns the `steps` order the certificate follows.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the `operation-contract` kind.
- [FR-354](../ir-proof/FR-354-record-proof-trust-basis.md) owns the checker outcome to certification table.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) owns the closed `certification` values, the `extent` wire form and the causes this requirement uses.
- QSL FR-146 (Claim extent) states the same roots and classification for the QSL request writer.
