---
id: TC-292
title: "Fairness over a supplied trace is a missing fairness premise"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-362
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-161
    type: verifies
---

# TC-292: Fairness over a supplied trace is a missing fairness premise

## Description

A supplied trace carries states and steps and no enabledness, so any fairness
constraint over it settles `unsupported`.

## Test Procedure

Supply the lasso L2 of TC-291 as a recorded trace with no model subject behind
it, and evaluate `always eventually holds(t.s = 2)` over it three times: with
`fair weak Cnt::T::q;`, with `fair strong each Cnt::T::q;`, and with no
fairness constraint. Then evaluate it with
`fair weak Cnt::T::p; fair strong Cnt::T::q;`.

## Expected Results

The weak and the strong clause each settle `unsupported` with FR-241 execution
`unsupported`, cause `unsupported_projection`/`missing-fairness-premise`,
naming their one constraint; the clause with no constraint evaluates to
`satisfied` on the lasso (FR-362-AC-8). The two-constraint clause names
(`weak`, `p`, `whole`), the first of its fairness set. Each `unsupported`
result stays distinct from `inconclusive` and from `failed` (FR-161-AC-3).
