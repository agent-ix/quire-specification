---
id: SR-13344
title: "Code review of QSpec PR #289 (STD-223) sampler vectors"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@61125de; spec/test-cases/TC-354-weighted-sampler-vectors.md, proposals/quire-v1/definitions/simulation-sampler.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/TC-354
    type: reviews
---

## Summary

Ticket: STD-223. Spec-only diff; no Rust, schema or vector JSON changed. Recomputed by hand and in Python: `n * floor(2^256/n)` is `2^256 - 1` at `n = 2^256 - 1`, `2^256` at `n = 2^256`, and `0` at `n = 2^256 + 1` and `n = 2^257`. The SM-08 preimage `{"choice":"0","draw":"0","seed":"424242","step":"0","trace":"0"}` hashes to `d5160380…995f2` (the TC-210/SM-01 digest), which is below `2^256 - 1`, so SM-08 selects index 0 at draw 0. SM-08 weights `(2^256 - 1, 1)` sum to `2^256`, SM-09 weights `(2^256, 1)` sum to `2^256 + 1`; both are correct. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (105 and 9).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| TC-354-SM-08 | spec/test-cases/TC-354-weighted-sampler-vectors.md | examined |
| TC-354-SM-09 | spec/test-cases/TC-354-weighted-sampler-vectors.md | examined |
| SAMPLER-selection | proposals/quire-v1/definitions/simulation-sampler.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | SM-08's expected result restates the rule ("index `0` if `v < 2^256 - 1`, else `1`") instead of the computed value, while TC-354's Expected Results requires every selected index "exactly as stated". The draw-0 digest is the known `d5160380d7495443315376de306a5d3613f3e010df74a5db92853829205995f2`, so state "accepted at `draw = 0` with that digest; selects index `0`". | spec/test-cases/TC-354-weighted-sampler-vectors.md:31,36 |

## Verdict

Mergeable on this lens after the wording nit; both vectors are correct.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | 42d2008: SM-08 now states acceptance at `draw = 0` with digest `d5160380…95f2` and selects index `0`. |
