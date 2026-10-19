---
id: SR-13347
title: "Integrity review of QSpec PR #289 (STD-223) sampler total bound"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@61125de; proposals/quire-v1/definitions/simulation-sampler.md, spec/functional/runtime/FR-181-simulate-finite-executions.md, spec/test-cases/TC-354-weighted-sampler-vectors.md; spec/objects/foundation/FR-272-typed-diagnostic-cause.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, proposals/backend-provider-v1/schema.json (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: reviews
---

## Summary

Ticket: STD-223. Checks the new refusal against the diagnostic cause catalog (FR-272, native-diagnostics), the other declared consumer of the sampler (FR-408 statistical checking) and the QSL half (agent-ix/quire-spec-language PR #1061, FR-188/FR-189/FR-193 at f376490). The bug is real: `n * floor(2^256 / n)` is 0 for every `n > 2^256` (recomputed for `n = 2^256 + 1` and `2^256 * 2`), so rejection sampling never accepts. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (105 and 9 tests) on the PR head; no schema or vector JSON is touched.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| SAMPLER-selection | proposals/quire-v1/definitions/simulation-sampler.md | examined |
| FR-181-AC-14 | spec/functional/runtime/FR-181-simulate-finite-executions.md | examined |
| TC-354-SM-09 | spec/test-cases/TC-354-weighted-sampler-vectors.md | examined |
| FR-272-AC-1 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-272-invalid_runtime_input-row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| DIAG-invalid_runtime_input-row | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| FR-408-sampling-stops | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md | examined |
| FR-331-unsupported-causes | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |
| BP1-DiagnosticCause | proposals/backend-provider-v1/schema.json | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `total-above-draw-range` is not a registered `invalid_runtime_input` cause. FR-272 declares the variant sets "closed and mandatory" and FR-272-AC-1 requires "exactly the closed variant set recorded above, with no added or removed variant"; the `invalid_runtime_input` row in FR-272 and in native-diagnostics lists twelve causes and not this one. The PR's new refusal therefore contradicts FR-272-AC-1. Neither row says what payload the cause retains (the row's generic payload is the input path and owning role; QSL's `TotalAboveDrawRange` carries the scaled `total`). Register the cause with its payload in both tables (FR-272 row and native-diagnostics row), or reuse an existing cause. The backend-provider-v1 `DiagnosticCause.cause` is a free kebab-case pattern, so the wire schema needs no change for this family. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:53,81,178; proposals/quire-v1/definitions/native-diagnostics.md:76; proposals/quire-v1/definitions/simulation-sampler.md:54-56 |
| FND-002 | medium | The statistical-checking consumer of the sampler is left without an outcome, and QSpec and QSL now disagree on the result kind. The sampler definition names FR-408 as a selecting consumer, and FR-408 "While sampling, the provider stops the run and settles the item `unsupported` with" lists `not-markov`, `not-stochastic` and `time-locked-sample` only. QSpec says the choice "refuses `invalid_runtime_input`/`total-above-draw-range`"; QSL #1061 FR-189 maps the same stop to `Unsupported(TotalAboveDrawRange{total})` and only FR-193 replay to `invalid_runtime_input`/`invalid-value`. Both sides agree on the sampler-level behaviour (refuse before any digest; `n = 2^256` selects). State FR-408's outcome; if it is `unsupported`, FR-331's `unsupported` cause list and the backend-provider-v1 `UnsupportedCause` schema need a variant (and `cargo test --test backend_provider_v1` then exercises it). | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:63-71; spec/objects/interfaces/FR-331-backend-provider-envelope.md:106; proposals/quire-v1/definitions/simulation-sampler.md:4-7,54-56 |

## Verdict

Not mergeable on this lens: FND-001 makes the PR contradict FR-272-AC-1. The sampler-level rule itself is consistent with the QSL half: same refusal point (before any digest, no choice index), same `2^256` boundary, and refusal rather than widening the draw.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | 42d2008: the condition is no longer an `invalid_runtime_input` cause; it is the `unsupported` result cause `total-above-draw-range{total}`, listed in FR-331's `unsupported` row and added to backend-provider-v1 `UnsupportedCause` with `total` a `PositiveIntegerString`. FR-272 and native-diagnostics list diagnostic-code causes only; no `unsupported` result cause (`not-markov`, `time-locked-sample`, …) appears in either, so FR-272-AC-1 is no longer touched and no registration there is needed. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (105, 9). |
| FND-002 | fixed | 42d2008: FR-408 "While sampling" gains `total-above-draw-range{total}` at the first such choice, matching QSL #1061 FR-189 `Unsupported(TotalAboveDrawRange{total})`. No vectors.json entry was added; no sibling `unsupported` sampling cause (`not-markov`, `not-stochastic`, `time-locked-sample`) has one either. The FR-331 `quire validate` failures (missing `id`, `semantic.record-invalid` on `fields`) are identical on origin/main 96e8fd7 and pre-exist. |
