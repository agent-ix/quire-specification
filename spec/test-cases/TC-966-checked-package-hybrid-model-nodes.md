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
declaration and `SlowEnough` claim. HN-01 to HN-10 read that file, and
HN-15 `positive-all-families.json`, in the QSpec harness
(`tests/checked_package_v2.rs`). HY-01 to HY-16 compile the
vectors through the real local Rust boundary; for HY-03 to HY-14 edit the
node of HY-01 as stated, adding to the graph any node the edit references,
then set each edited node's `dependencies` to its body's targets, recompute
its key and every key that references it, and recompute the `package_id`
over the derived identity projection, as TC-233 does for its patched
vectors, and feed the package to the reader.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HN-01 | the published nodes | every node meets `SemanticNode` and holds the key of its own preimage; each `hybrid` node's preimage meets `HybridNode`, the node is its own `semantic_type` and its `dependencies` are its body's targets; FR-481-AC-1's model, modes, variables and classes, `Moving`'s rates `-a`, `0` and `1`, its transition's target 1, guard `v = 0` and reset `(0, 0)`; the clock `t`'s rate is the rational literal `1` in both modes; FR-481-AC-6's `SlowEnough` |
| HN-02 | `Moving`'s rate of `v` replaced by `-(2 * a)`, then rekeyed up the references | the added term nodes hold their own keys; new keys for `Moving`, `Braking`, `h` and `SlowEnough`; rekeying the unchanged nodes keeps every key |
| HN-03 to HN-10 | the `schema_refusals` cases: a transition with a `declaration`; a mode occurrence of role `claim`; `modes` before `variables`; class `continuous`; a reference `target`; a mode with no rates; form `hybrid_flow`; a model without `owner` | each fails `SemanticNode`, so a reader refuses it `malformed_wire` at strict wire validation |
| HN-15 | `positive-all-families.json`'s `Tank` | a `hybrid_model`, a `hybrid_mode` and a `hybrid_transition` node, each holding its own key; `x`'s rate and `x`'s reset value reference `y`'s variable node; `y`, classed `hybrid`, has the rate literal `1`; the full reader admits the package |
| HY-01 | `Braking` and `SlowEnough` of QSL FR-447, compiled and emitted | the HN-01 nodes; the full reader admits the package |
| HY-02 | HY-01 respelled with other whitespace and comments | one `package_id` with HY-01 |
| HY-03 | `modes` listing `Moving` twice | `invalid_semantic_graph` at the second entry |
| HY-04 | `v`'s parameter node typed `scalar_type`/`integer`; `a`'s at level 2 | `invalid_model_binding`/`malformed-declaration` at the target, each |
| HY-05 | `Moving` with a fourth rate | `invalid_model_binding`/`malformed-declaration` at `rates` |
| HY-06 | `Moving`'s invariant an integer-typed node; `Moving`'s rate of `v` the node of `v = 0` | `ill_typed`/`non-boolean-root`; `ill_typed`/`type-mismatch` |
| HY-07 | the transition's `target` 2 | `missing_declaration`/`missing-name` at `target` |
| HY-08 | a second reset entry for variable 0 | `invalid_package`/`conflicting-definition` naming both entries |
| HY-09 | the guard `w = 0` over an added parameter node `w` at level 3 | `missing_declaration`/`missing-name` at `guard` |
| HY-10 | `Stopped`'s rate of `t` the node of `2 - 1` with `t` classed `clock`; every variable classed `clock`; a one-mode model `Timer` whose only variable `t` is classed `clock`, with rate `1` and invariant `t <= 5` | `invalid_package`/`invalid-value` at `t`'s class literal; at `v`'s class literal; at `Timer`'s `variables` |
| HY-11 | a second mode node declared `[Braking, Moving]` in `modes` | `ambiguous_declaration`/`ambiguous-name` |
| HY-12 | an added transition node no mode references | `invalid_semantic_graph` at that node |
| HY-13 | `SlowEnough` with clock `model-steps`; with an `each` activation | `invalid_package`/`invalid-value` at `/body/arguments/1`; at `/body/arguments/2` |
| HY-14 | `SlowEnough`'s formula `once [0, 2] holds(v = 0)`; a `holds` operand reading `h` | `ill_typed`/`operator-ineligible` at the formula node; `missing_declaration`/`missing-name` at the `holds` node |
| HY-15 | HY-01's v2 bytes handed to a process fixture provider for `SlowEnough`'s item | the provider reads the model and claim from the nodes, derives horizon 10, and the item's extent is `bounded` |
| HY-16 | the process fixture provider's `solver-inconclusive` traces over HY-01, read by the host: `Moving` then `Stopped` by `transition` 0; mode `Halted` | the first kept on the `inconclusive` record; the second refused by the host `invalid_result`/`invalid-value` at its `mode` |
| HY-21 | `SlowEnough` under a `quire.temporal.timed/v1` profile alias, its interval in the timed form `[0, 10]` with both ends closed | admitted, its law `timed/v1` |

## Expected Results

The hybrid model, its modes, transitions, variables and classes and each
claim over it are graph nodes whose keys carry every member, each malformed
node refuses at the first failing schema or reader step, and a process
provider serves the claim from the package bytes alone.
