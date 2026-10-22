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
compare node ids and identities pairwise. For SN-08 to SN-14, edit the node
of SN-01, SN-02 or SN-03 as stated; except in SN-12, set its `dependencies`
to the body's targets; then recompute its node key, rewrite its
`identity_projection` entry and recompute the `package_id`, as TC-233 does
for its patched vectors, and feed the package to the reader.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SN-01 | TC-331 PV-01, `possible ReachesTwo using v over (c: Config::ConfigVersion) { c.versionNumber = 2 }` | one node, with no `declaration` and no `owner`; body `kind` `possible`, `over` a reference to `c`'s parameter node, `from` the empty aggregate, `target` a reference to the `expression` node of `c.versionNumber = 2`; occurrence role `claim`; `dependencies` the two targets in digest order |
| SN-02 | TC-332 AP-02, `always possible CanStillWin using v over (x: G::Game) from (x.phase != G::Phase::Lost) { x.phase = G::Phase::Won }` | `kind` `always-possible`; `from` references the `expression` node of `x.phase != G::Phase::Lost` |
| SN-03 | TC-333 UP-01, `unique path InOneWay` | `kind` `unique-path`; `from` references `X`'s node and `target` `Y`'s node |
| SN-04 | `possible Anything using v { true }` | admitted; `over` the empty aggregate; `target` a reference to the Boolean `value`/`literal` node of `true`; `dependencies` that node only |
| SN-05 | SN-02 without `from`, and SN-02 with `from (true)` | one node id; `from` references the `true` literal node |
| SN-06 | SN-01, and in a separate unit `always possible AlwaysReachesTwo using v over (c: Config::ConfigVersion) { c.versionNumber = 2 }` | different node ids and different obligation identities |
| SN-07 | SN-01's two instances, `c = a` and `c = b`; SN-01 with different whitespace and comments | the two instances have different obligation identities; the respelled claim has SN-01's node id and identities |
| SN-08 | SN-02 with `from` and `target` swapped in order | `invalid_semantic_graph` at the body |
| SN-09 | SN-01 with `kind` `eventually-possible` | `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{n}/body/members/0` |
| SN-10 | SN-01 with `from` a reference to its target node | `invalid_semantic_graph` at the `from` binding |
| SN-11 | SN-01 with `target` a reference to `c`'s parameter node | `ill_typed`/`operator-ineligible` at the target |
| SN-12 | SN-01 with its target node removed from `dependencies` | `invalid_semantic_graph` at the node's `/dependencies` |
| SN-13 | SN-03 with `from` the empty aggregate | `invalid_semantic_graph` at the `from` binding |
| SN-14 | SN-01 with `over` the text literal `c` | `invalid_semantic_graph` at the `over` binding |
| SN-15 | one unit declaring SN-01's claim twice, as `ReachesTwo` and `AlsoReachesTwo` | one node with two `claim` occurrences, ordinals 0 and 1 |
| SN-16 | SN-01's `c = a` instance requested over §7.1's subject with initial state `(0, 0)` and with initial state `(1, 0)` | one node id; two different obligation identities |

## Expected Results

Each form has its own body and identity, spelling never changes identity, and
each malformed node refuses at the first failing reader step.
