---
id: TC-334
title: "State-graph partial runs, verdict map and reductions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-391
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
---

# TC-334: State-graph partial runs, verdict map and reductions

## Description

Check which evidence settles a state-graph item on a partial exploration, the verdict map when no evidence is decisive, the FR-360 combination of every label, and the reduction rows.

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine under the stated limits, replay any evidence, and record the label, its FR-241, FR-242 and FR-243 values, the cause and the settlement method. Each refutation and each witnessed proof is checked by replaying its evidence, never by comparing it with one expected evidence value; the verdict, basis, settlement method and cause are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| PR-01 | an `always possible` claim whose initial state reaches, at depth 2, a terminal node where `Q` holds and `P` does not, on a model whose other branch counts over `Int[0, 1000]`, maximum depth 4 | `refuted`, `closed-scope`, with a `trap`; the open nodes at depth 4 do not prevent it |
| PR-02 | a `possible` claim whose `P` node lies at depth 6, with no trap, maximum depth 4, `witness_samples` 0 | `inconclusive`, `unsettled`, `bound-reached` naming depth 4 |
| PR-03 | PR-02 with `max_states` set below the reachable count | `incomplete`, cause `limit-reached` naming `max_states`, its value and the setting that raises it |
| PR-04 | a `possible` claim witnessed at depth 2 on every initial state, run with `max_states` that stops exploration before it completes | `inconclusive`, cause `well-definedness-unchecked`, carrying the witnesses and stating that well-definedness was not checked; never `proved` |
| PR-05 | an `always possible` claim with no trap and open nodes from a state constraint | `inconclusive`, `constraint-reached` |
| PR-06 | a `possible` claim on a model whose expansion evaluates a contract conjunction undecided | `inconclusive`, `undecided-successor` |
| PR-07 | a `possible` claim on a subject with no initial state | `inconclusive`, `no-initial-state` |
| PR-08 | an `always possible` claim with partial-order reduction selected | `inconclusive`, `reduction-not-preserving`, before expansion |
| PR-09 | a `unique path` claim with an admitted symmetry selected | `inconclusive`, `reduction-not-preserving`, before expansion |
| PR-10 | TC-331 PV-01 under an admitted symmetry over `{a, b}`, and under partial-order reduction with `versionNumber` writes visible | `proved`, `decisive-witness`, sources named, no reduction named; the witness carries no canonical state |
| PR-12 | PR-02 with the caller cancelling the run before exploration completes | `incomplete`, cause `cancelled`; FR-360 `failed`, execution `resource-incomplete`, basis `unavailable` |
| PR-13 | a `possible` claim `over (c: Config::ConfigVersion)` on a subject whose universe is empty | `inconclusive`, `no-instance`, before expansion |
| PR-11 | every result of PR-01 to PR-10, PR-12 and PR-13 | each label carries exactly the FR-241, FR-242 and FR-243 combination FR-360 names |

## Expected Results

A partial exploration settles an item only from decisive evidence; otherwise it settles by the verdict map and never as `proved` or `refuted`.
