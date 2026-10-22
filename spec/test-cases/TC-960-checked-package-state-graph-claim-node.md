---
id: TC-960
title: "Checked-package state-graph claim node and obligation identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-475
    type: verifies
---

# TC-960: Checked-package state-graph claim node and obligation identity

## Description

Check the `claim`/`state_graph` node body of each state-graph form, the
obligation identity that separates forms and instances, and the reader's
refusals of a malformed node.

## Test Procedure

Compile each vector through the real local Rust boundary, read the
`claim`/`state_graph` node, and compute each item's obligation identity;
compare node ids and identities pairwise. For SN-08 to SN-12, edit the node
of SN-01 or SN-02 as stated, recompute its node key, and feed the package to
the reader.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SN-01 | TC-331 PV-01, `possible ReachesTwo using v over (c: Config::ConfigVersion) { c.versionNumber = 2 }` | one node; body `kind` `possible`, `over` a reference to `c`'s parameter node, `from` the empty aggregate, `target` a reference to the `expression` node of `c.versionNumber = 2`; occurrence role `claim`; `dependencies` the two targets in digest order |
| SN-02 | TC-332's `always possible CanStillWin using v over (x: G::Game) from (x.phase != Lost) { x.phase = Won }` | `kind` `always-possible`; `from` references the `expression` node of `x.phase != Lost` |
| SN-03 | TC-333's `unique path InOneWay` | `kind` `unique-path`; `from` references `X`'s node and `target` `Y`'s node |
| SN-04 | `possible Anything using v { true }` | `over` the empty aggregate; `dependencies` the target only |
| SN-05 | SN-02 without `from`, and SN-02 with `from (true)` | one node id |
| SN-06 | SN-01 and `always possible ReachesTwo using v over (c: Config::ConfigVersion) { c.versionNumber = 2 }` | different node ids and different obligation identities |
| SN-07 | SN-01's two instances, `c = a` and `c = b`; SN-01 with different whitespace and comments | the two instances have different obligation identities; the respelled claim has SN-01's node id and identities |
| SN-08 | SN-02 with `from` and `target` swapped in order | `invalid_semantic_graph` at the body |
| SN-09 | SN-01 with `kind` `eventually-possible` | `invalid_package`/`invalid-value` at the `kind` binding |
| SN-10 | SN-01 with `from` a reference to its target node | `invalid_semantic_graph` at the `from` binding |
| SN-11 | SN-01 with `target` a reference to `c`'s parameter node | `ill_typed`/`operator-ineligible` at the target |
| SN-12 | SN-01 with its target node removed from `dependencies` | `invalid_semantic_graph` at the node's `/dependencies` |

## Expected Results

Each form has its own body and identity, spelling never changes identity, and
each malformed node refuses at the first failing reader step.
