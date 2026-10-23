---
id: TC-967
title: "Checked-package schedulability claim node and obligation identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-482
    type: verifies
---

# TC-967: Checked-package schedulability claim node and obligation identity

## Description

Check the `claim`/`schedulability` node body of a `schedulable` and a
`response` claim, the obligation identity that separates policies and
premises and ignores spelling, and the reader's refusals of a malformed node.

## Test Procedure

Compile each vector through the real local Rust boundary, read the
`claim`/`schedulability` node, and compute each item's obligation identity;
compare node ids and identities pairwise. Every unit declares the dimension
`Time` and the unit `ms` of it, and TC-372's `Ctl` task set; `CtlFp`, `CtlEdf`
and `T3` are TC-372's claims. For SC-08 to SC-19, edit the node of SC-01 or
SC-02 as stated, recompute its node key and recompute the `package_id` over
the derived identity projection, as TC-233 does for its patched vectors, and
feed the package to the reader.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SC-01 | `CtlFp` | one node, `semantic_form` `schedulability`, no `declaration` and no `owner`; body `kind` `schedulable`, `policy` `fixed-priority`, `task` and `bound` the empty aggregate, `tasks` the aggregates of `t1`, `t2`, `t3` in that order, each `arrival` `periodic`, `period` and `deadline` `4`, `6`, `12`, `wcet` `1`, `2`, `5`, `wcet_hi` the empty aggregate, `criticality` `lo`, `priority` `1`, `2`, `3`, `jitter` and `blocking` `0`, every time value typed by the `ms` unit node; one occurrence of role `claim`; `dependencies` `[]` |
| SC-02 | `T3` | `kind` `response`, `policy` `fixed-priority`, `task` the text `t3`, `bound` the `quantity` literal `12` |
| SC-03 | `CtlFp` and `CtlEdf` | different node ids and different obligation identities |
| SC-04 | `CtlFp` over `Ctl` with `t3`'s `wcet 6 ms` | a different node id and obligation identity |
| SC-05 | `CtlFp` over `Ctl` with its tasks listed `t3`, `t1`, `t2`; with `jitter 0 ms` added to `t1`; with other whitespace and comments; with the claim named `Other` | SC-01's node id and obligation identity for each |
| SC-06 | one unit declaring `CtlFp`'s claim twice, as `CtlFp` and `CtlFp2` | one node with two `claim` occurrences, ordinals 0 and 1 |
| SC-07 | `Ctl` with `t1` written `hi wcet_hi 2 ms` under `schedulable CtlAmc on Ctl under amc;` | `policy` `amc`; `t1`'s `criticality` `hi` and `wcet_hi` the `quantity` literal `2` |
| SC-08 | SC-01 with `tasks` holding no member | `invalid_semantic_graph` at `tasks` |
| SC-09 | SC-01 with `policy` and `kind` swapped in order | `invalid_semantic_graph` at the body |
| SC-10 | SC-01 with `policy` `rate-monotonic` | `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{n}/body/members/1` |
| SC-11 | SC-01 with `kind` `response` | `invalid_semantic_graph` at the `task` binding |
| SC-12 | SC-01 with `t1`'s `wcet_hi` the `quantity` literal `2` and `criticality` `lo` | `invalid_semantic_graph` at `t1`'s `wcet_hi` binding |
| SC-13 | SC-01 with its tasks in the order `t2`, `t1`, `t3` | `invalid_semantic_graph` at `tasks` |
| SC-14 | SC-02 with `task` the text `t9` | `invalid_semantic_graph` at the `task` binding |
| SC-15 | SC-01 with `t2`'s `period` typed by a unit of a dimension `Length` | `ill_typed`/`type-mismatch` at that literal |
| SC-16 | SC-01 with `t1`'s `period` `0` | `invalid_timed_model`/`invalid-task-set` at `t1`'s aggregate |
| SC-17 | SC-01 with `t2`'s `jitter` `-1` | `invalid_timed_model`/`invalid-task-set` at `t2`'s aggregate |
| SC-18 | SC-01 with `t3`'s `priority` `1`; the same edit of `CtlEdf`'s node | `invalid_timed_model`/`invalid-task-set` at `t3`'s aggregate; admitted |
| SC-19 | SC-01 with `dependencies` naming the `ms` unit node | `invalid_semantic_graph` at the node's `/dependencies` |

## Expected Results

Each claim has its own body and identity, every stated premise and the
policy enter it, spelling never changes it, and each malformed node refuses
at the first failing reader step.
