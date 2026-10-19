---
id: SR-13326
title: "Failure-domain review of QSpec PR #288 (STD-208)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@07eb2ff; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md, spec/functional/tooling/FR-306-compile-and-cache-exactly.md, spec/functional/tooling/FR-462-speak-the-plugin-wire.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-373-hybrid-verdict-map.md; ticket STD-208"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-306
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-373
    type: reviews
---

## Summary

Ticket: STD-208. PR: quire-specification#288 at 07eb2ff. Unstated failure modes and identity confusion around the step budget, the time budget and the provider deadline `deadline_ms`, including the plugin wire (FR-462), since an external hybrid solver can be a plugin provider.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-193-row-step | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-row-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-steps | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-identity | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-AC-8 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-AC-9 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-331-incomplete-row | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-hybrid-solver-budgets | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-request-identity | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-AC-29 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-306-outside-key | spec/functional/tooling/FR-306-compile-and-cache-exactly.md | examined |
| TC-373-HY-04 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| TC-373-HY-07 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| TC-373-HY-08 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| FR-462-AC-5 | spec/functional/tooling/FR-462-speak-the-plugin-wire.md | context_only |
| FR-331-options | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-193 now says a run ended by the caller's `deadline_ms` settles `cancelled{source: deadline}`, "never `timed-out`" (verdict-map row, AC-8, HY-07), but FR-462-AC-5 and FR-462's Failures say a plugin's deadline overrun "ends the process and settles each open item `incomplete` with cause `timed-out`", and FR-331 defines `deadline_ms` as the provider deadline (FR-462). A hybrid solver served by a plugin therefore has two required causes for the same event. Resolve in this PR: either change FR-462 (and TC-428) so a deadline overrun settles `cancelled{source: deadline}`, matching FR-331's incomplete row, or restrict FR-193's "never `timed-out`" to in-process providers. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:76,114; spec/functional/tooling/FR-462-speak-the-plugin-wire.md:122-123,154; spec/test-cases/TC-373-hybrid-verdict-map.md:32 |
| FND-002 | medium | Where rounding happens and what the wire carries are unstated. `options` is a string-valued map (FR-331, schema StringMap) and FR-331 refuses out-of-domain values and non-canonical encodings before consumption, yet FR-193/FR-331 say a fractional `max_solver_time` is "rounded up when the request is admitted". It is not said whether the request writer rounds before writing (QSL FR-250 says the request writer does) or the provider rounds a received "1.5", whether a fractional string on the wire is admitted or refused, or how whole seconds and whole steps are spelled in the string ("2" vs "2s" vs "2.0"). AC-9's "admitted as 2 s" has no fixed boundary to test at. State the wire form (decimal digits, no unit) and that the wire carries only whole seconds, with rounding at the request writer. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:87-92,115; spec/objects/interfaces/FR-331-backend-provider-envelope.md:192,212-222 |

## Verdict

Not mergeable: FND-001 is a direct contradiction with FR-462 for plugin-served hybrid solvers. FND-002 leaves AC-9 without a testable boundary. The step/time split itself is sound: steps are deterministic and in identity, wall time is outside identity and never stored.
