---
id: TC-197
title: "Systems-model typed binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-152
    type: verifies
---

# TC-197: Systems-model typed binding

## Description

Resolve every model element kind and attempt kind, direction and package substitutions.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-152. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select `quire.model.complete/v1` at revision `1-draft.1` by its exact
DefinitionRefs in
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json).
Node names, declaration keys, package identity, multiplicity notation and
default limits follow TC-195 and TC-196; the source is TC-195 S1 importing Y,
with profile alias `V` and model alias `M`. A binder request is written
`resolve(kind, name)`. Each systems node is declared by one artifact of its
kind: `part`, `port`, `interface`, `connection` or `allocation`.

Fixture **Y** (domain package `test/orders`, version `y`):

- value type `Count`, bound as `Int[0,9]`;
- object types `Sys`, `Pump`, `Tank`; interfaces `Flow` and `Flow2`, each with
  one feature, the field `Flow/rate` typed `Count` `{1,1}`, and
  `Flow2.supertypes = [Flow]`;
- Parts: `Sys/pump` (owner `Sys`, type `Pump`, `{1,1}`) and `Sys/tank` (owner
  `Sys`, type `Tank`, `{1,1}`);
- Ports: `Sys/pump/out` (owner part `Sys/pump`, interface `Flow`, direction
  `out`, `{1,1}`) and `Sys/tank/in` (owner part `Sys/tank`, interface `Flow`,
  direction `in`, `{1,1}`);
- Connection: `Sys/pipe`, source end `Sys/pump/out`, target end `Sys/tank/in`,
  both `{1,1}`, `direction: source-to-target`;
- operation `Pump/run` with no params, no returns and an empty frame;
- Allocation: `Pump/alloc`, source end `Pump/run`, target end `Sys/pump`;
- navigation: relationship `rel/parts` of `Sys` with source end role `owner`
  (`Sys`, `{1,1}`), target end role `parts` (`Pump`, `{0,3}`), direction
  `source-to-target`; and relationship `rel/home` of `Pump` with source end role
  `pump` (`Pump`, `{1,1}`), target end role `home` (`Tank`, `{1,1}`), direction
  `source-to-target`.

Population **Q**: population declaration with member types `[Sys, Pump, Tank]`
and extent `closed`; runtime object `s1` of type `Sys` whose `parts` instances
target `p2` and `p1`, objects `p1`, `p2` of type `Pump` whose `home` targets
`t1` of type `Tank`.

Ownership: the Parts, both Ports and the Connection `Sys/pipe` (both ends
owned by `Sys`) are effective members of `Sys`; the Allocation's source
end-owner type is `Pump` (owner of `Pump/run`) and its target end-owner type is
`Sys` (owner of `Sys/pump`), so it is qualified into both and resolves as the
member owned by `Pump`.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| Y01 | `resolve(Part, M::Sys::pump)`, `resolve(Port, M::Sys::pump::out)`, `resolve(Interface, M::Flow)`, `resolve(Connection, M::Sys::pipe)`, `resolve(Allocation, M::Pump::alloc)` | each returns its kind with the exact declaration key and effective declaration identity: Part `(Sys, Sys/pump)`, Port `(Sys, Sys/pump/out)`, Interface the effective type `Flow`, Connection `(Sys, Sys/pipe)` and Allocation `(Pump, Pump/alloc)`; `M::Sys::alloc` does not name the Allocation, which is exposed only as the member owned by its source end-owner type; the Connection carries source `Sys/pump/out` and target `Sys/tank/in`; the Allocation carries source `Pump/run` and target Part `Sys/pump` |
| Y02 | `resolve(Port, M::Sys::pump)`; `resolve(Allocation, M::Sys::pipe)`; Y with `Pump/alloc`'s target end replaced by `Sys/pump/out` | `refused { code: invalid_model_binding, cause: wrong-export }` with required `Port`, actual `Part`; the same with required `Allocation`, actual `Connection`; the same for the allocation target with required `Part`, actual `Port` |
| Y03 | Y with `Sys/pipe` changed to: (a) source `Sys/tank/in`, target `Sys/pump/out`; (b) direction `target-to-source`; (c) direction `bidirectional`; (d) (c) with both ports `inout`; (e) direction `undirected` | (a) `refused { code: invalid_model_binding, cause: port-direction }` with direction `source-to-target`, source `in`, target `out`; (b) the same refusal: the flow source is the target end `in`; (c) the same refusal: both ports must be `inout`; (d) admitted, both interface types being `Flow`; (e) the same refusal, `undirected` never being a connection direction |
| Y04 | Y with `Sys/tank/in` interface `Flow2`; then instead `Sys/pump/out` interface `Flow2` | `refused { code: ill_typed, cause: type-mismatch }` with flow-source interface `Flow` not conforming to `Flow2`, the interface condition charging `f(Flow) = 1` work unit, so the three `systems.connection-condition` charges consume three work units; admitted, `Flow2` conforming to `Flow`, the interface condition charging `f(Flow2) = 2` (its qualify and inherit facts), so the three charges consume four work units |
| Y05 | Y with `Sys/pipe`'s target end multiplicity `{0,2}` | `refused { code: ill_typed, cause: multiplicity-narrowing }` with expected port multiplicity `{1,1}` and actual end `{0,2}`; no connection is exposed |
| Y06 | Y with `Sys/pump/out`'s owner changed to the object type `Pump` | after all `systems.kind` and `systems.allocation` charges, two `invalid_model_binding` refusals in rule order, each naming the node, its artifact and its span: (1) kind mapping, `wrong-export` for port `Sys/pump/out` with required `Part` and actual `none`; (2) kind mapping, `wrong-export` for the source end of `Sys/pipe` with required `Port` and actual `none`, so `Sys/pipe` has no kind and no `systems.connection-condition` charge; `resolve(Port, M::Sys::pump::out)` has no result |
| Y07 | Over Q, `self.parts` in `invariant I using V on M::Sys at current { ... }` with `self` = `s1`; then with every `ScalarLimitsV1` counter unlimited except `work_units: 5`, then `4`; then Q plus objects `p3` and `p4` of type `Pump` with `s1.parts` targeting `p1`, `p2`, `p3`, `p4`; then the same `s1.parts` targets with `p4` absent from the closed population | `Set<Reference<M::Pump>>[0,3]` equal to `[p1, p2]` in reference-key order, after `model.navigate` (`value_occurrences` 1), two `collection.visit`, `collection.bound` (`value_occurrences` 2) and `collection.result-retain` (`value_occurrences` 3, `result_units += 3`): five work units and three result units; the `5` run completes identically; `incomplete { limit_kind: work_units, limit: 4, consumed: 4, next_charge: 1, charge_point: collection.result-retain }`; `refused { code: cardinality_out_of_bound, cause: above-maximum }` with bound `[0,3]` and count 4 after `model.navigate`, four `collection.visit` and `collection.bound`: exactly six work units and no result unit, with no `collection.result-retain` and no value; `refused { code: dangling_reference, cause: absent-target-in-complete-population }` for `p4` after `model.navigate` and three `collection.visit` (`p1`, `p2`, `p3`): exactly four work units, decided before `collection.bound`, so no cardinality refusal |
| Y08 | `self.home` with `self` = `p1`; `self.pump` with `self` = `t1` in an invariant on `M::Tank`; Y with `rel/home` direction `undirected`, then `self.pump` again; Y with an added field `Pump/home` typed `Tank`, then `self.home` | `Reference<M::Tank>` equal to `t1` after one `model.navigate`: one work unit and no result unit; `refused { code: ill_typed, cause: operator-ineligible }`, the traversal running target to source against `source-to-target`; the navigation checks, `undirected` admitting both traversals, and evaluating it for `t1` is `refused { code: cardinality_out_of_bound, cause: above-maximum }` with bound `[1,1]` and count 2 (`p1` and `p2`) after one `model.navigate`; `refused { code: ambiguous_declaration, cause: ambiguous-name }` listing the field and the relationship end |
| Y09 | Y with `rel/parts` target end `{0,unbounded}`; then `{0,3}` with `ordered: true`; then Q with `p2`'s `home` targeting `t9` absent from the closed population; then Y with `rel/home`'s target end naming `ix://test/other/Tank`, a node of another domain package | `refused { code: unsupported_construct, cause: expression-form }` at `self.parts`; the same refusal; `refused { code: dangling_reference, cause: absent-target-in-complete-population }` for `t9`; `refused { code: missing_declaration, cause: missing-name }` at intake naming `rel/home`, the absent node `ix://test/other/Tank`, the artifact and the span, with no effective view |

## Expected Results

Every positive and boundary result matches FR-152; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
