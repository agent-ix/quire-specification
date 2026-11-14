---
id: SR-202611141457003
title: "Criterion strength review of the STD-665 statistical.workers registration"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-specification; PR #654 at a5e9aa28, diff origin/main...HEAD: spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md (Inputs, Workers section, FR-408-AC-15..17), spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md (Behavior 5), spec/objects/interfaces/FR-331-backend-provider-envelope.md (request member list), spec/test-cases/TC-356-regenerative-long-run.md (WK-01..04); compared with QSL FR-191 and FR-255 on agent-ix/quire-spec-language main"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-408, type: references }
  - { target: ix://agent-ix/quire-specification/FR-461, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
  - { target: ix://agent-ix/quire-specification/TC-356, type: references }
---

## Summary

Ticket: STD-665. Jev is not installed on this host (`jev not found`), so the Jev-calibrated
judgement was skipped and the criteria were judged manually against spec text only.

FR-408-AC-15 can fail: any count, estimate, witness, provenance, certificate, decision or stop that
differs from the one-worker run at `workers` 2, 4 or 64, a result that changes when the sampler
double completes a higher trace index first, or a sample beyond index 99 counted under
`max_samples` 100. FR-408-AC-16 can fail: accepting 0, a different code or cause, a refusal that
does not name `statistical.workers`, any recorded draw, a default other than 1, or a
`StatisticalLimits` row or `limit-reached` name for workers. FR-408-AC-17 can fail: a settled state
other than `incomplete`/`cancelled{source}`, counts or estimates that are not a one-worker prefix at
the reached index (the provenance trace-index range makes that index observable), or a carried
sample above it.

## Verdict

PASS with two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-408-AC-15 has no adverse case on physical draws: unlike QSL FR-191-AC-6 ("the double records at most 103 draws"), nothing fails for a provider that draws far beyond the stop and discards the excess (see base FND-002) | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:358 |
| FND-002 | low | FR-408-AC-16's clause "`statistical.workers` takes no row in `StatisticalLimits` and no `limit-reached` name" has no TC-356 vector (WK-03 checks only the refusal and the default), and WK-03 names no fixture or method | spec/test-cases/TC-356-regenerative-long-run.md:39 |
