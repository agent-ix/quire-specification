---
id: TC-197
title: "Systems-model typed binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-152
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-208
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-197: Systems-model typed binding

## Description

Resolve every model element kind and attempt kind, direction and package substitutions.

## Test Procedure

Use the subject and inputs named by FR-152. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select `quire.model.complete/v1` through
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json).
Node names, declaration keys, package identity, multiplicity notation and
default limits follow TC-195 and TC-196; the source is TC-195 S1 importing Y,
with profile alias `V` and model alias `M`. A binder request is written
`resolve(kind, name)`. Every type definition and the population are declared
by their own artifact, and each node's identity is `ix://test/orders/<artifact
id>`. Y's `constructs` table has these entries, all under module `test/arch`:
kind `object` with meaning `quire.meaning.model.object-type/v1`; `value` with
`quire.meaning.model.value-type/v1`; `population` with
`quire.meaning.model.population/v1`; and `interface`, `part`, `port`,
`connection` and `allocation` with `quire.meaning.systems.interface/v1`,
`quire.meaning.systems.part/v1`, `quire.meaning.systems.port/v1`,
`quire.meaning.systems.connection/v1` and
`quire.meaning.systems.allocation/v1`. Each node below names the kind its role
states. Y declares no variant type.

Fixture **Y** (domain package `test/orders`, version `y`):

- value type `Count` (kind `value`), bound as `Int[0,9]`;
- object types `Sys`, `Pump`, `Tank`; interfaces `Flow` and `Flow2`, each with
  one feature, the field `Flow/rate` typed `Count` `{1,1}`, and
  `Flow2.supertypes = [Flow]`;
- Parts: artifacts `sys_pump` (`owner: Sys`, type `Pump`, `{1,1}`) and
  `sys_tank` (`owner: Sys`, type `Tank`, `{1,1}`);
- Ports: artifacts `pump_out` (`owner: sys_pump`, interface `Flow`, direction
  `out`, `{1,1}`) and `tank_in` (`owner: sys_tank`, interface `Flow`, direction
  `in`, `{1,1}`);
- Connection: artifact `pipe`, source end `pump_out`, target end `tank_in`,
  both `{1,1}`, `direction: source-to-target`;
- operation `Pump/run` with no params, no returns and an empty frame;
- Allocation: artifact `pump_alloc`, source end `Pump/run`, target end
  `sys_pump`;
- navigation: relationship `Sys/composition` with source end role `parts`
  (`Sys`, `{1,1}`), target end role `owner` (`Pump`, `{0,3}`), direction
  `source-to-target`; and relationship `Pump/housing` with source end role
  `home` (`Pump`, `{1,1}`), target end role `pump` (`Tank`, `{1,1}`), direction
  `source-to-target`. A role names the end nearest the receiver (FR-152
  "Navigation"): `self.parts` navigates from a `Sys` to the `Pump` end and
  `self.home` from a `Pump` to the `Tank` end, and the inverse names `owner`
  and `pump` navigate from a `Pump` and a `Tank`.

Population **Q**: population declaration from artifact `Q` (kind `population`) with member types `[Sys, Pump, Tank]`
and extent `closed`; runtime object `s1` of type `Sys` whose `parts` instances
target `p2` and `p1`, objects `p1`, `p2` of type `Pump` whose `home` targets
`t1` of type `Tank`. Q's observation lists `Sys/composition` as a complete extent
with the instances `s1`-`p2` and `s1`-`p1`, and `Pump/housing` as a complete
extent with `p1`-`t1` and `p2`-`t1`; a vector that names another extent states it.

Ownership: the Parts, both Ports and the Connection `pipe` (both ends owned by
`Sys`) are effective members of `Sys`, keyed `(Sys, <construct key>)`; the
Allocation's source end-owner type is `Pump` (owner of `Pump/run`) and its
target end-owner type is `Sys` (owner of `sys_pump`), so it is qualified into
both and resolves as the member owned by `Pump`. No owner or end appears in any
construct's identity.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| Y01 | `resolve(Part, M::sys_pump)`, `resolve(Port, M::pump_out)`, `resolve(Interface, M::Flow)`, `resolve(Connection, M::pipe)`, `resolve(Allocation, M::pump_alloc)` | each returns its kind with the exact declaration key and effective declaration identity: Part `(Sys, sys_pump)`, Port `(Sys, pump_out)`, Interface the effective type `Flow`, Connection `(Sys, pipe)` and Allocation `(Pump, pump_alloc)`; `M::Sys::pump_alloc` and `M::Sys::sys_pump` name nothing, because a construct is named by its artifact id and never through an owner; the Connection carries source `pump_out` and target `tank_in`; the Allocation carries source `Pump/run` and target Part `sys_pump`; kind mapping charges eight `systems.kind`, in order `Flow`, `Flow2`, `sys_pump`, `sys_tank`, `pump_out`, `tank_in`, `pipe`, `pump_alloc` |
| Y02 | `resolve(Port, M::sys_pump)`; `resolve(Allocation, M::pipe)`; Y with `pump_alloc`'s target end replaced by `pump_out` | `refused { code: invalid_model_binding, cause: wrong-export }` with required `Port`, actual `Part`; the same with required `Allocation`, actual `Connection`; the same for the allocation target with required `Part`, actual `Port` |
| Y03 | Y with `pipe` changed to: (a) source `tank_in`, target `pump_out`; (b) direction `target-to-source`; (c) direction `bidirectional`; (d) (c) with both ports `inout`; (e) direction `undirected` | (a) `refused { code: invalid_model_binding, cause: port-direction }` with direction `source-to-target`, source `in`, target `out`; (b) the same refusal: the flow source is the target end `in`; (c) the same refusal: both ports must be `inout`; (d) admitted, both interface types being `Flow`; (e) the same refusal, `undirected` never being a connection direction |
| Y04 | Y with `tank_in` interface `Flow2`; then instead `pump_out` interface `Flow2` | `refused { code: ill_typed, cause: type-mismatch }` with flow-source interface `Flow` not conforming to `Flow2`, the interface condition charging `f(Flow) = 1` work unit, so the three `systems.connection-condition` charges consume three work units; admitted, `Flow2` conforming to `Flow`, the interface condition charging `f(Flow2) = 2` (its qualify and inherit facts), so the three charges consume four work units |
| Y05 | Y with `pipe`'s target end multiplicity `{0,2}` | `refused { code: ill_typed, cause: multiplicity-narrowing }` with expected port multiplicity `{1,1}` and actual end `{0,2}`; no connection is exposed |
| Y06 | Y with `pump_out`'s owner changed to the object type `Pump` | after all `systems.kind` and `systems.allocation` charges, two `invalid_model_binding` refusals in rule order, each naming the node and its `origin`: (1) kind mapping, `wrong-export` for port `pump_out` with required `Part` and actual `none`; (2) kind mapping, `wrong-export` for the source end of `pipe` with required `Port` and actual `none`, so `pipe` has no kind and no `systems.connection-condition` charge; `resolve(Port, M::pump_out)` has no result |
| Y07 | Over Q, `self.parts` in `invariant I using V on M::Sys at current { ... }` with `self` = `s1`; then with every `ScalarLimitsV1` counter unlimited except `work_units: 5`, then `4`; then Q plus objects `p3` and `p4` of type `Pump` with `s1.parts` targeting `p1`, `p2`, `p3`, `p4`; then the same `s1.parts` targets with `p4` absent from the closed population | `Set<Reference<M::Pump>>[0,3]` equal to `[p1, p2]` in reference-key order, after `model.navigate` (`value_occurrences` 1), two `collection.visit`, `collection.bound` (`value_occurrences` 2) and `collection.result-retain` (`value_occurrences` 3, `result_units += 3`): five work units and three result units; the `5` run completes identically; `incomplete { limit_kind: work_units, limit: 4, consumed: 4, next_charge: 1, charge_point: collection.result-retain }`; `refused { code: cardinality_out_of_bound, cause: above-maximum }` with bound `[0,3]` and count 4 after `model.navigate`, four `collection.visit` and `collection.bound`: exactly six work units and no result unit, with no `collection.result-retain` and no value; `refused { code: dangling_reference, cause: absent-target-in-complete-population }` for `p4` after `model.navigate` and three `collection.visit` (`p1`, `p2`, `p3`): exactly four work units, decided before `collection.bound`, so no cardinality refusal |
| Y08 | `self.home` with `self` = `p1`; `self.pump` with `self` = `t1` in an invariant on `M::Tank`; Y with `Pump/housing` direction `undirected`, then `self.pump` again; Y with an added field `Pump/home` typed `Tank`, then `self.home` | `Reference<M::Tank>` equal to `t1` after one `model.navigate`: one work unit and no result unit; `refused { code: ill_typed, cause: operator-ineligible }`, the traversal running target to source against `source-to-target`; the navigation checks, `undirected` admitting both traversals, and evaluating it for `t1` is `refused { code: cardinality_out_of_bound, cause: above-maximum }` with bound `[1,1]` and count 2 (`p1` and `p2`) after one `model.navigate`; `refused { code: ambiguous_declaration, cause: ambiguous-name }` listing the field and the relationship end |
| Y09 | Y with `Sys/composition` target end `{0,unbounded}`; then `{0,3}` with `ordered: true`; then Q with `p2`'s `home` targeting `t9` absent from the closed population; then Y with `Pump/housing`'s target end naming `ix://test/other/Tank`, a node of another domain package | `refused { code: unsupported_construct, cause: expression-form }` at `self.parts`; the same refusal; `refused { code: dangling_reference, cause: absent-target-in-complete-population }` for `t9`; `refused { code: missing_declaration, cause: missing-name }` at intake naming `Pump/housing`, the absent node `ix://test/other/Tank` and the referring node's `origin`, with no effective view |
| Y10 | Y plus a `constructs` entry for kind `{module: test/other, name: port}` with meaning `quire.meaning.model.object-type/v1`, an entry for kind `{module: test/other, name: socket}` with meaning `quire.meaning.systems.port/v1`, and object type `Probe` (artifact `Probe`) of kind `{module: test/other, name: port}`; `resolve(Port, M::Probe)`; then that package with `pipe`'s target end replaced by `Probe`; then Y plus those two `constructs` entries and `Probe`, with `pipe` unchanged and `tank_in`'s kind changed to `{module: test/other, name: socket}`, whose meaning is the same Port meaning | `Probe` has no systems kind and no `systems.kind` charge; `refused { code: invalid_model_binding, cause: wrong-export }` with required `Port` and actual `none`; kind mapping refuses the target end of `pipe` with `wrong-export`, required `Port`, actual `none`, so `pipe` has no kind; the renamed node maps to Port exactly as in Y01, with the same charges and declaration keys |
| Y11 | `self.owner` with `self` = `s1` in an invariant on `M::Sys`; `self.parts` with `self` = `p1` in an invariant on `M::Pump`; `self.pump` with `self` = `p1` in an invariant on `M::Pump` | each `refused { code: ill_typed, cause: type-mismatch }` at the form with no evaluation charge: `owner` is the target end's role, navigable only from a `Pump`; `parts` is the source end's role, navigable only from a `Sys`; `pump` is a target-end role navigable only from a `Tank`. Checking source reports the cause an undeclared field gets; a `relationship_end` member naming one of these roles on the wrong endpoint is refused by the IR checks of FR-322-AC-56 and AC-57 instead |
| Y12 | Y07's `self.parts` lowered to a checked package | the `quire.op.model.navigate` application has operator class `query`, member kind `relationship_end` naming `Sys/composition` and role `parts`, one operand of type `Reference<M::Sys>` and `result_type` `Set<Reference<M::Pump>>[0,3]` |
| Y13 | `self.parts` with `self` = `s1`, over Q with the `Sys/composition` extent unlisted; then listed `"complete": false` with the instances `s1`-`p1` and `s1`-`p2`; then listed complete with no instance; then `self.home` with `self` = `p1` over Q with `Pump/housing` listed complete with no instance | the first two each `incomplete { code: incomplete_population, cause: incomplete-scope }` retaining `Sys/composition` and `s1`, after `model.navigate` alone: one work unit, no `collection.visit`, `collection.bound` or `collection.result-retain`, and no value; the complete empty extent returns the empty `Set<Reference<M::Pump>>[0,3]` after `model.navigate`, `collection.bound` and `collection.result-retain`: three work units, the bound `[0,3]` holding for count 0; `refused { code: cardinality_out_of_bound, cause: below-minimum }` with bound `[1,1]` and count 0 after `model.navigate` alone |
| Y14 | Y plus the relationship `Pump/feeds`: source role `feeds`, target role `fedBy`, both ends `Pump`, direction `source-to-target`, target end `{0,3}` unique, with the objects `p1`, `p2`, `p3` and the complete extent `p1`-`p2`, `p1`-`p3`, `p2`-`p3`. In an invariant on `M::Pump`: `reaches(p1, p3, feeds)`, `reaches(p3, p1, feeds)` and `reaches(p1, p1, feeds)`; then the first with `work_units: 5`, then `4`; then the third with `value_occurrences: 3`, then `2` | `true` after `graph.expand` for `p1`, `graph.edge` to `p2`, `graph.expand` for `p2`, `graph.edge` to `p3` and `graph.result-retain`: five work units and one result unit; `false` after `graph.expand` for `p3` and `graph.result-retain`: two work units; `false` after three `graph.expand` (`p1`, `p2`, `p3`; `value_occurrences` 1, 2, 3), three `graph.edge` (`p1`-`p2`, `p1`-`p3`, `p2`-`p3`) and `graph.result-retain`: seven work units, because no link returns to `p1`; the `5` run completes identically; `incomplete { limit_kind: work_units, limit: 4, consumed: 4, next_charge: 1, charge_point: graph.result-retain }`; the `3` run completes; `incomplete { limit_kind: value_occurrences, limit: 2, consumed: 2, next_charge: 3, charge_point: graph.expand }` |
| Y15 | Y14 with `Pump/feeds` target end `unique: false` and the extent `p1`-`p2` twice and `p1`-`p3`: `reaches(p1, p3, feeds)`; then the extent with `p1`-`p4` as a fourth link of `p1` over the bound `[0,3]` and `reaches(p1, p3, feeds)` | `true` after `graph.expand` for `p1`, `graph.edge` to `p2`, `graph.expand` for `p2`, a second `graph.edge` to `p2` (discovered once), `graph.edge` to `p3` and `graph.result-retain`: six work units; `true` with the same charges, the neighbour count 4 above the bound refusing nothing and no `cardinality_out_of_bound` |
| Y16 | Y14 with the `Pump/feeds` extent unlisted, then listed `"complete": false` with the links of Y14, then listed complete with no link, then complete with `p1`-`p9` where `p9` is absent from the closed population and `reaches(p1, p9, feeds)`, then the extent `p1`-`p2` with `reaches(p1, p3, feeds)` over a `Pump` population listed `open` whose only known member is `p1` (so `p2` is neither present nor known absent), then `reaches(p2, p1, feeds)` over that same population, then `reaches(p9, p1, feeds)` over the complete population without `p9` | the first two each `incomplete { code: incomplete_population, cause: incomplete-scope }` retaining `Pump/feeds` and `p1` after `graph.expand` for `p1` alone: one work unit and no Boolean, though a known link would reach the target; `false` after `graph.expand` and `graph.result-retain`: two work units; `refused { code: dangling_reference, cause: absent-target-in-complete-population }` for `p9` after `graph.expand` for `p1` alone, before the `graph.edge`; `incomplete_population`/`incomplete-scope` retaining `p2` after `graph.expand` for `p1` and no `graph.edge`; `incomplete_population`/`incomplete-scope` retaining `p2` with no charge at all, the undecidable start deciding first; `refused { code: dangling_reference, cause: absent-target-in-complete-population }` for `p9` with no charge at all |
| Y17 | `reaches(s1, s1, parts)` over Y, where `parts` targets `Pump`; Y14 with `Pump/feeds` target end `{0,unbounded}`; with target end `{0,3}` `ordered: true`; with `Pump/feeds` direction `target-to-source` and `reaches(p1, p3, feeds)`; with `Pump/feeds` direction `target-to-source`, target end `{0,unbounded}` | `ill_typed`/`operator-ineligible` at `member.name`, the destination end not being a `Sys`; `unsupported_construct`/`expression-form` at `member.name`; the same; `ill_typed`/`operator-ineligible` at `member.name`, the direction; the same direction refusal, direction being decided before multiplicity |
| Y18 | Y14's `reaches(p1, p3, feeds)` lowered to a checked package, then the same over a `[0,1]` end | one `quire.op.model.reaches` application with operator class `reaches`, member kind `relationship_end` naming `Pump/feeds` and role `feeds`, operands of type `Reference<M::Pump>` and `result_type` `Boolean`, the same shape for both multiplicities |
| Y19 | Y14 with the `Pump/feeds` target end `{2,3}` and the complete extent `p1`-`p2` and `p1`-`p3`, `p2`-`p3`: `reaches(p2, p3, feeds)` and `reaches(p3, p2, feeds)`; then `reaches(p2, p3, feeds)` with `work_units: 3`, then `2` | `true` after `graph.expand` for `p2`, `graph.edge` to `p3` and `graph.result-retain`: three work units, one result unit, `p2` holding one neighbour below the lower bound 2 and no `cardinality_out_of_bound`; `false` after `graph.expand` for `p3` and `graph.result-retain`: two work units, `p3` holding none; the `3` run completes; `incomplete { limit_kind: work_units, limit: 2, consumed: 2, next_charge: 1, charge_point: graph.result-retain }` |

## Expected Results

Every positive and boundary result matches FR-152; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
