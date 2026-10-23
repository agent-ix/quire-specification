---
id: TC-966
title: "Checked-package hybrid model nodes and hybrid claims"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-481
    type: verifies
---

# TC-966: Checked-package hybrid model nodes and hybrid claims

## Description

Check the `hybrid` nodes a `hybrid-decl` produces and the
`temporal`/`temporal_clause` node of a claim over it, that every member of
the model enters identity, the published schema's and the reader's refusals
of a malformed node, and that a process provider serves the claim from the
package bytes.

## Test Procedure

`hybrid-node-vectors.json` holds the nodes of QSL FR-447's `Braking`
declaration and `SlowEnough` claim, and as `restored_nodes` those of
`Braking2`, `Slows` and `Recent`. HN-01 to HN-14 read that file in the
QSpec harness (`tests/checked_package_v2.rs`). HY-01 to HY-20 compile the
vectors through the real local Rust boundary; for HY-03 to HY-14 and HY-17
to HY-19 edit the
node of HY-01 as stated, adding to the graph any node the edit references,
then set each edited node's `dependencies` to its body's targets, recompute
its key and every key that references it, and recompute the `package_id`
over the derived identity projection, as TC-233 does for its patched
vectors, and feed the package to the reader.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HN-01 | the published nodes | every node meets `SemanticNode` and holds the key of its own preimage; each `hybrid` node's preimage meets `HybridNode`, the node is its own `semantic_type` and its `dependencies` are its body's targets; FR-481-AC-1's model, modes, transition, variables and classes; the clock `t`'s rate is the rational literal one in both modes; `SlowEnough` is a `temporal_clause` over `h` typed by the model node, clock `model-time` |
| HN-02 | `Moving`'s rate of `v` replaced by `-(2 * a)`, then rekeyed up the references | new keys for `Moving`, `Braking`, `h` and `SlowEnough`; rekeying the unchanged nodes keeps every key |
| HN-03 to HN-10 | the `schema_refusals` cases: a transition with a `declaration`; a mode occurrence of role `claim`; `modes` before `variables`; class `continuous`; a reference `target`; a mode with no rates; form `hybrid_flow`; a model without `owner` | each fails `SemanticNode` |
| HN-11 | the `restored_nodes` with `nodes` as one graph | every restored node meets `SemanticNode` and holds the key of its own preimage, each `hybrid` one self-typed with its body's targets as `dependencies`; FR-481-AC-8's `initial` and reset relation; `Braking` has no `initial` binding |
| HN-12 | `Slows` and `Recent` | FR-481-AC-9's clock, activation, trigger and read levels, capture, fairness member and intervals |
| HN-13 | `Braking2`'s model with an empty `initial`; an initial entry without `condition`; `T2`'s reset `relation` `approx` | each fails `SemanticNode` |
| HN-14 | `Slows`'s `eventually` interval with `upper` `null` and `upper_end` `open` | meets `SemanticNode` |
| HY-01 | `Braking` and `SlowEnough` of QSL FR-447, compiled and emitted | the HN-01 nodes; the full reader admits the package |
| HY-02 | HY-01 respelled with other whitespace and comments | one `package_id` with HY-01 |
| HY-03 | `variables` and `modes` swapped | `invalid_semantic_graph` at the body |
| HY-04 | `v`'s parameter node typed `scalar_type`/`integer`; `a`'s at level 2 | `invalid_model_binding`/`malformed-declaration` at the target, each |
| HY-05 | `Moving` with a fourth rate | `invalid_model_binding`/`malformed-declaration` at `rates` |
| HY-06 | `Moving`'s invariant an integer-typed node; `Moving`'s rate of `v` the node of `v = 0` | `ill_typed`/`non-boolean-root`; `ill_typed`/`type-mismatch` |
| HY-07 | the transition's `target` 2 | `missing_declaration`/`missing-name` at `target` |
| HY-08 | a second reset entry for variable 0 | `invalid_package`/`conflicting-definition` naming both entries |
| HY-09 | the guard `w = 0` over an added parameter node `w` at level 3 | `missing_declaration`/`missing-name` at `guard` |
| HY-10 | `Stopped`'s rate of `t` the node of `2 - 1` with `t` classed `clock`; every variable classed `clock` | `invalid_package`/`invalid-value` at the class literal; at `variables` |
| HY-11 | a second mode node declared `[Braking, Moving]` in `modes` | `ambiguous_declaration`/`ambiguous-name` |
| HY-12 | an added transition node no mode references | `invalid_semantic_graph` at that node |
| HY-13 | `SlowEnough` with clock `wall`; with an `each` activation whose `trigger` is typed by the `scalar_type`/`rational` node | `invalid_package`/`invalid-value` at `/body/arguments/1`; `ill_typed`/`type-mismatch` at the trigger |
| HY-14 | `SlowEnough`'s formula `once [0, 2] holds(v = 0)`; a `holds` operand reading `h` | admitted; `missing_declaration`/`missing-name` at the `holds` node |
| HY-15 | HY-01's v2 bytes handed to a process fixture provider for `SlowEnough`'s item | the provider reads the model and claim from the nodes, derives horizon 10, and the item's extent is `bounded` |
| HY-16 | the provider's `solver-inconclusive` traces over HY-01: `Moving` then `Stopped` by `transition` 0; mode `Halted` | the first kept on the `inconclusive` record; the second `invalid_result`/`invalid-value` at its `mode` |
| HY-17 | `Braking2`, `Slows` and `Recent` of QSL FR-447, compiled and emitted; then `Braking` with the written default `initial`; an initial `mode` 2; an initial condition referencing the literal `20` | the HN-11 and HN-12 nodes, the reader admitting the package; `invalid_semantic_graph` at `initial`; `missing_declaration`/`missing-name`; `ill_typed`/`non-boolean-root` |
| HY-18 | `T2`'s reset `relation` `eq`; a reset `(0, 0)` beside `(0, 1, ge)` | `invalid_package`/`invalid-value` at the relation; `invalid_package`/`conflicting-definition` naming both entries |
| HY-19 | `Slows`'s fairness member naming `Halted`; a `holds` operand reading a parameter `v` at level 4 | `missing_declaration`/`missing-name` at the member; at the `holds` node |
| HY-20 | a timed interval with `upper` `null` in `Slows` under the infinite-trace profile; the same interval in a temporal clause over a state model | admitted; refused as FR-370 refuses an interval its profile does not admit |

## Expected Results

The hybrid model, its modes, transitions, variables and classes and each
claim over it are graph nodes whose keys carry every member, each malformed
node refuses at the first failing schema or reader step, and a process
provider serves the claim from the package bytes alone.
