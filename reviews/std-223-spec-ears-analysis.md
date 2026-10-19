---
id: SR-13346
title: "EARS review of QSpec PR #289 (STD-223) FR-181-AC-14"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@61125de; spec/functional/runtime/FR-181-simulate-finite-executions.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: reviews
---

## Summary

Ticket: STD-223. EARS conformance of the one requirement-grammar statement the diff adds, FR-181-AC-14. The sampler definition paragraph is a definition, not a requirement statement, and is out of this lens.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-181-AC-14 | spec/functional/runtime/FR-181-simulate-finite-executions.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-14 joins two obligations with a semicolon (the refusal above `2^256` and the boundary at exactly `2^256`) and the second one, "selects normally", names no observable outcome. Split it, and state the boundary as "a total of exactly `2^256` accepts the draw-0 digest and selects the least index `i` with `v < w_0 + … + w_i`" (TC-354 SM-08). | spec/functional/runtime/FR-181-simulate-finite-executions.md:265 |

## Verdict

Minor wording only; no EARS defect blocks merge.
