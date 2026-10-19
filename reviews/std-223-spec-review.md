---
id: SR-13345
title: "Spec review of QSpec PR #289 (STD-223) sampler total bound"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@61125de; proposals/quire-v1/definitions/simulation-sampler.md, spec/functional/runtime/FR-181-simulate-finite-executions.md, spec/test-cases/TC-354-weighted-sampler-vectors.md, reviews/std-223-spec-review-base.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: reviews
---

## Summary

Ticket: STD-223. Base checklist over the sampler-definition paragraph, FR-181-AC-14 and TC-354 SM-08/SM-09. The defect is confirmed: for `n > 2^256`, `floor(2^256 / n) = 0`, so no `v` is accepted and the existing rule loops. Refusing (rather than widening the draw) keeps the sampler identity and every digest sequence unchanged. AC-14 is a free id (AC-8 is retired, AC-13 exists). The PR also adds `reviews/std-223-spec-review-base.md` (SR-13328), written by the PR's author; it is not an independent review and its only finding row records an owner decision, not a defect.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| SAMPLER-selection | proposals/quire-v1/definitions/simulation-sampler.md | examined |
| FR-181-AC-14 | spec/functional/runtime/FR-181-simulate-finite-executions.md | examined |
| TC-354-SM-08 | spec/test-cases/TC-354-weighted-sampler-vectors.md | examined |
| TC-354-SM-09 | spec/test-cases/TC-354-weighted-sampler-vectors.md | examined |
| SR-13328 | reviews/std-223-spec-review-base.md | examined |
| FR-181-AC-11 | spec/functional/runtime/FR-181-simulate-finite-executions.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `invalid_runtime_input` is the wrong family for this condition. Elsewhere in QSpec it refuses supplied input (a supplied snapshot, a replayed trace whose delay or digest is wrong, FR-181-AC-11) and its payload is "the actual input path, owning required role and supplied mapping". The scaled total comes from the model's declared workload or parameter weights over the enabled set at a generated state; no supplied input is wrong and no input path exists. QSL #1061 uses `invalid_runtime_input` only for witness replay (FR-193) and `Unsupported` for sampling. Choose the family for generated sampling (an `unsupported` cause, or a model-side family) and keep `invalid_runtime_input` for replay of a supplied trace. | proposals/quire-v1/definitions/simulation-sampler.md:54-56; spec/functional/runtime/FR-181-simulate-finite-executions.md:263,265 |
| FND-002 | low | "such a choice refuses" does not say what the refusal ends. A choice is not a request: FR-181 does not state whether the whole simulation request refuses (discarding traces already generated) or only that trace stops, and whether the refusal names the trace, step and choice position. The paragraph's neighbour, "An empty item set ends the trace without a draw", is per-trace. State the scope and the locus retained. | proposals/quire-v1/definitions/simulation-sampler.md:54-59; spec/functional/runtime/FR-181-simulate-finite-executions.md:265 |

## Verdict

Not mergeable as-is together with SR-13347 FND-001 (high). On this lens the rule is right in substance: refusal before any digest, no choice index, `n = 2^256` selecting with bound `2^256`, and no widening. The self-authored SR-13328 should be replaced by the independent reviewer artifacts (SR-13344 to SR-13347).
