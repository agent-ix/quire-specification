---
id: TC-198
title: "Closed model lookup"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-153
    type: verifies
---

# TC-198: Closed model lookup

## Description

Query lookup/all-instances with complete, unknown, foreign and over-bound populations.

## Test Procedure

Use the subject and inputs named by FR-153. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select `quire.model.complete/v1` through
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json).
Node names, declaration keys, package identity and default limits follow
TC-195. The model is TC-195 F1 imported as `M` by TC-195 S1's shape with
profile alias `V`: object universe `U` = `af5934e1`, effective types
`A` = `4531dd9e` and `B` = `c5264509`. Because `4531dd9e` precedes `c5264509`,
every `A` member precedes every `B` member in reference-key order.

F1 also declares population **PA**, from artifact `PA` with IR node identity
`ix://test/orders/PA`, whose construct meaning is
`quire.meaning.model.population/v1`, with member types `[A, B]` and extent
`closed`. A runtime member record names its object
identity and the declaration key of its most-specific type.

Population binding `p` has role `population`, anchor `current`, declared
maximum 3 and is bound to PA of F1's ModelSelection with runtime members
**P1**: `a1` and `a2` of type `A` and `b1` of type `B`. Binding `q` is the same
shape over PA with runtime members **P2**: one member `c9` of type `A`. `ra`, `rb` and `rc` are
`Reference<M::A>`, `Reference<M::B>` and `Reference<M::A>` parameters valued
`a1`, `b1` and `c9`.

Population binding `p0` has the same role, anchor, PA declaration,
ModelSelection and P1 runtime members as `p`, but declares no maximum.
PA establishes object and subtype closure for `M::A`; with adequate admission
and evaluation limits, `allInstances<M::A>(p0)` returns the unbounded
`Set<Reference<M::A>>` containing `[a1, a2, b1]` in canonical reference-key
order, retaining the most-specific triples `(U, A, "a1")`, `(U, A, "a2")`
and `(U, B, "b1")`. It has no inferred finite maximum.

Graph population binding `g` has role `population`, anchor `current`, declared
maximum 3 and is bound to PA of F1's ModelSelection with runtime members
**P3**: `a1` of type `A` with `x` targeting `a2`, `a2` of type `A` with `x`
absent, and `b1` of type `B` with `x` targeting `b1`.
`ga1`, `ga2` and `gb1` are `Reference<M::A>` parameters valued `a1`, `a2` and
`b1` of P3, and `gz` is a `Reference<M::A>` parameter valued `z9`, absent from
P3. Every `ScalarLimitsV1` counter not stated is unlimited.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| L01 | `allInstances<M::A>(p)`; then with every `ScalarLimitsV1` counter unlimited except `work_units: 5`, then `4`; `allInstances<M::B>(p)` | `Set<Reference<M::A>>[0,3]` equal to `[a1, a2, b1]`, where `b1`'s triple is `(U, B, "b1")`, after three `population.visit`, `collection.bound` (`value_occurrences` 3) and `collection.result-retain` (`value_occurrences` 4, `result_units += 4`): five work units and four result units; the `5` run completes identically; `incomplete { limit_kind: work_units, limit: 4, consumed: 4, next_charge: 1, charge_point: collection.result-retain }`; `[b1]` of type `Set<Reference<M::B>>[0,3]` after three `population.visit` (every member walked, `a1` and `a2` not selected), the bound and a retain of two occurrences: five work units and two result units, with `b1`'s triple identical to its triple in the `M::A` result |
| L02 | PA with extent `open`; then PA with member types `[A]` and P1 without `b1`, and `allInstances<M::A>(p)` | binding admission returns the incomplete outcome `incomplete_population` with cause `incomplete-scope`, not a refusal, and no charge or collection; the incomplete outcome `incomplete_population` with cause `unclosed-subtypes` naming F1's ModelSelection, PA, `M::A` and the uncovered type `B`, with no query charge or collection |
| L03 | `lookup<M::A>(p, rb) absent undefined`; `lookup<M::A>(p, rc) absent undefined`; `lookup<M::A>(p, rb) absent empty`; `lookup<M::A>(p, rc) absent empty`; `lookup<M::A>(p, rc) absent refused`; `lookup<M::B>(p, ra) absent empty` | `b1` as `Reference<M::A>` after `lookup.key` and `lookup.result-retain`: two work units, one result unit; `undefined` with the catalogued reason `absent-key` after one work unit; present `b1` as `Option<Reference<M::A>>` after two work units and two result units; `none` after two work units and one result unit; `refused { code: invalid_runtime_input, cause: absent-key }` naming `p` and `c9` after one work unit; `refused { code: ill_typed, cause: type-mismatch }` before any charge, `A` not conforming to `B` |
| L04 | `lookup<M::A>(p, rx) absent empty` where `rx` is valued `a1` from a closed population of TC-195 F2 | `refused { code: foreign_reference, cause: foreign-universe }` with required `af5934e1` and supplied `6763207e`, after one work unit |
| L05 | P1 plus a second member record `a1` of type `B`; P1 plus an exact duplicate of member `a1` with an equal member digest, then `allInstances<M::A>(p)`; P1 plus member `z1` of type `ix://test/orders/Z`, not covered by PA's member types | binding admission `refused { code: invalid_runtime_input, cause: conflicting-identity }` listing both `a1` records, after four `binding.member` charges (`population_members` 4) and decided after the fourth; the duplicate collapses at the fourth `binding.member` charge, admission consumes four admission work units, and L01's evaluation result and charges recur; `refused { code: foreign_reference, cause: foreign-type }` for `ix://test/orders/Z` at binding admission after four `binding.member` charges |
| L06 | (1) `p` with declared maximum 2, then `allInstances<M::A>(p)`; (2) the same with only `work_units: 3`; (3) `allInstances<M::A>(p0)` over PA/P1 with no declared maximum and adequate admission/evaluation limits; (4) `allInstances<M::Count>(p)` for a scalar type | (1) `refused { code: cardinality_out_of_bound, cause: above-maximum }` with bound `[0,2]` and count 3, after three visits and `collection.bound`: four work units and no result unit; (2) `incomplete { limit_kind: work_units, limit: 3, consumed: 3, next_charge: 1, charge_point: collection.bound }`; (3) unbounded `Set<Reference<M::A>>` equal to `[a1, a2, b1]` in canonical reference-key order, with triples `(U, A, "a1")`, `(U, A, "a2")`, `(U, B, "b1")`, after three `population.visit`, `collection.bound` (`value_occurrences` 3) and `collection.result-retain` (`value_occurrences` 4, `result_units += 4`): five work units and four result units, no finite default or cardinality refusal; (4) `refused { code: ill_typed, cause: type-mismatch }` |
| L07 | F1 plus operation `A/remove` with no params, no returns and frame `{modifies: [], creates: [], deletes: [A]}`; an invocation deleting `a2`, whose pre population is P1 and post population is P1 without `a2`; in `post Q using V on M::A::remove { ... }`: `allInstances<M::A>(p)`, `pre(allInstances<M::A>(p))`, `lookup<M::A>(p, r2) absent empty` and `pre(lookup<M::A>(p, r2) absent empty)` for `r2` valued `a2` | `[a1, b1]`; `[a1, a2, b1]` with `a2`'s triple `(U, A, "a2")`; `none`; present `a2` with its pre type `A` |
| L08 | `(lookup<M::A>(p, rb) absent refused) = rb`; then over TC-195 F2 with `rb: Reference<M::B>` and `rk: Reference<M::C>`, `rb = rk` | `true` through the FR-149 `Reference<M::B>` to `Reference<M::A>` upcast, which charges nothing: two lookup work units and one result unit, then the five-unit reference equality: seven work units and two result units; `refused { code: ill_typed, cause: type-mismatch }` before any charge, neither type conforming to the other |
| L09 | Sort the `reference_key_order` entries of the vectors file as FR-144 keys, starting from any permutation | exactly the published order: `(6763207e, 4531dd9e, "o1")`, `(af5934e1, 4531dd9e, "o")`, `(af5934e1, 4531dd9e, "o1")`, `(af5934e1, 4531dd9e, "o10")`, `(af5934e1, 4531dd9e, "o2")`, `(af5934e1, c5264509, "o1")`: universe first, then type, then object bytes with a proper prefix first; each identity component compared by its FR-204 key bytes (the UTF-8 domain string followed by its 32 digest bytes, one byte string) and `object` by its exact UTF-8 bytes |
| L10 | Over `g`, `reaches(ga2, ga2, M::A::x)`, `reaches(gb1, gb1, M::A::x)`, `reaches(ga1, ga2, M::A::x)` and `reaches(ga1, ga1, M::A::x)`; then the first with `work_units: 1`, the second with `work_units: 2`, and the fourth with `value_occurrences: 1` | `false` after `graph.expand` for `a2` and `graph.result-retain`: two work units and one result unit (the start is discovered but not reached, and `a2` has no `x` edge); `true` after `graph.expand` for `b1`, one `graph.edge` to `b1` and `graph.result-retain`: three work units; `true` after `graph.expand` for `a1`, one `graph.edge` to `a2` and `graph.result-retain`: three work units; `false` after `graph.expand` for `a1` (`value_occurrences` 1), `graph.edge` to `a2`, `graph.expand` for `a2` (`value_occurrences` 2) and `graph.result-retain`: four work units, because no edge returns to `a1` (FR-043-AC-2); `incomplete { limit_kind: work_units, limit: 1, consumed: 1, next_charge: 1, charge_point: graph.result-retain }`; `incomplete { limit_kind: work_units, limit: 2, consumed: 2, next_charge: 1, charge_point: graph.result-retain }`; `incomplete { limit_kind: value_occurrences, limit: 1, consumed: 1, next_charge: 2, charge_point: graph.expand }`; no incomplete run exposes a Boolean |
| L11 | Over `g`, `deref(ga1).x`, then with `work_units: 1`; `deref(gz)`, then with `work_units: 0` | present `a2` as `Option<Reference<M::A>>` after `model.deref` and `model.navigate`: two work units and no result unit; `incomplete { limit_kind: work_units, limit: 1, consumed: 1, next_charge: 1, charge_point: model.navigate }`; `refused { code: dangling_reference, cause: absent-target-in-complete-population }` for `z9` after `model.deref`: one work unit; `incomplete { limit_kind: work_units, limit: 0, consumed: 0, next_charge: 1, charge_point: model.deref }` |
| L12 | PA2: PA's extent and ModelSelection with member types `[A]` only, over runtime members P1 (`a1`, `a2` of type `A`, `b1` of type `B`, where `B` is a direct subtype of `A`); the identity check covers `a1` and `a2` before any edge, so only `b1`'s coverage walk follows an edge, exactly one, `B` to `A`. Run with `PopulationAdmissionLimitsV1` counters not stated unlimited and `ancestor_steps: 1`, then `ancestor_steps: 0`; then `population_members: 3, work_units: 2` with `ancestor_steps: 0` | At 1 the population is admitted; at 0 exactly incomplete(ancestor_steps,0,0,1,admission.ancestor-edge) at stage `population-admission` and no population; with `work_units: 2` the `binding.member` charge of `b1` is denied first, incomplete(work_units,2,2,1,binding.member), and no edge is followed |
| L13 | A package whose header selects no `quire.model.complete/v1` profile and writes a `Population<T>[N]` binding type, `allInstances<T>(p)` and a record-field navigation `r.f`, each beside a `lookup`, a dispatched call and an `operation-body` | the `lookup`, dispatched call and `operation-body` each refuse `unsupported_construct`; the `Population<T>[N]` type, `allInstances` and the navigation raise no `unsupported_construct` and are admitted or refused only by the package's layer selection |
| L14 | F1 imported as alias `M`; PA's canonical member type is `A`, the least of `[A, B]` (`4531dd9e` precedes `c5264509`), with no `bound` on PA. In `invariant I using V on M::A at current` the body `count<M::A>(o in allInstances<M::A>(M::PA) : true) = 3` over the L07 current snapshot (P1); in `pre Q using V on M::A::remove` the body `allInstances<M::A>(M::PA)` and `lookup<M::A>(M::PA, r2) absent empty` over the pre snapshot (P1); in `post R using V on M::A::remove` the body `allInstances<M::A>(M::PA)`, `lookup<M::A>(M::PA, r2) absent empty` and `pre(lookup<M::A>(M::PA, r2) absent empty)`, over pre P1 and post P1 without `a2`. Then the refusals: `allInstances<M::A>(M::Nope)`; `M::PA = M::PA`; `let z = M::PA in allInstances<M::A>(z)`; `size(M::PA)`; a clause body that is `M::PA`; alias `M` also an enum name; the same `allInstances<M::A>(M::PA)` in a function body. Then PA with extent `open`; a snapshot with no PA row | `I` is `true` over `[a1, a2, b1]`; `Q` reads the pre population `[a1, a2, b1]` and the lookup is present `a2` with pre type `A`; `R` reads `[a1, b1]`, the lookup is `none` and the `pre(...)` lookup is present `a2`; each operand is typed `Population<M::A>` (no bracket); `missing_declaration`/`missing-name` at `Nope`; the next four each `ill_typed`/`operator-ineligible` at the name; `ambiguous_declaration`/`ambiguous-name` at the name; `missing_declaration`/`missing-name` in the function body; `incomplete_population`/`incomplete-scope`; `incomplete_population`/`missing-required-artifact`, each with no collection |

## Expected Results

Every positive and boundary result matches FR-153; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
