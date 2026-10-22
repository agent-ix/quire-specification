---
id: SR-14685
title: "gap analysis of STD-109 new ACs against TC rows (QSpec PR #340)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@102b3433fba0642b48b42b05524f60bb31a0512a; spec/functional/runtime/FR-181-simulate-finite-executions.md, spec/test-cases/TC-210-finite-simulation-reproducibility.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: reviews
---
# SR-14685: gap analysis of STD-109 (QSpec PR #340)

## Summary

Ticket: STD-109. Plan completion: not assessed. This is a manual mapping of
each new AC to its TC rows. The PR is spec only. The TC-210 harness
assertions are listed as remaining work for the code lane.

- FR-181-AC-16 maps to the first bullet of TC-210 "Successor and
  initial-state order vectors". It covers both successor orders and the
  coalesced equal post-state.
- FR-181-AC-17 maps to the second bullet: `[b, a]` against `[a, b]`, and
  `[a, b, a]` giving two initial states at indices 0 and 1.
- FR-181-AC-18 maps to the third bullet: traces 0 to 3 start at `a`, `b`,
  `a`, `b`, with no start draw. The one-initial-state case is the existing
  sampler index-sequence vector.
- The FR-361 "Model subject" sentence adds no AC. Its rule is FR-181-AC-17's.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. Each new AC has a TC-210 vector with a concrete expected result.
