---
id: SR-13322
title: "Base checklist review of QSpec PR #288 (STD-208)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@07eb2ff; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md, spec/functional/tooling/FR-306-compile-and-cache-exactly.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-373-hybrid-verdict-map.md; ticket STD-208"
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

Ticket: STD-208. PR: quire-specification#288 at 07eb2ff. Base checklist (id formats, duplicates, AC-to-TC links, validation) over `git diff origin/main...07eb2ff`. New ids FR-193-AC-8, FR-193-AC-9 and FR-331-AC-29 parse in `quire matrix --scope .` (untagged, like every other FR-193 criterion; QSpec has no engine to tag). No collision with open QSpec PRs: #287 (STD-211) adds FR-375/376/377 criteria and TC-310/311/315/316 rows; #286 is merged (FR-149); #200 reserves FR-331-AC-24..27, which this PR skips; #283 and #207 touch FR-331/FR-306 text but add no FR-193, FR-331-AC-29 or TC-373 ids. `quire validate --scope .` on the four changed files: FR-193, FR-306 and TC-373 pass; FR-331 fails with `[value_object] required 'id' is missing` and `semantic.record-invalid ... "fields" is a required property`, byte-identical on an unmodified `git archive origin/main` (2c2918b) copy, so pre-existing. Each new criterion names TC-373, and TC-373 now verifies FR-331.

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

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-331-AC-29's last clause (two requests differing only in `max_solver_time` have equal request identity) is exercised only by HY-08, but HY-08 cites FR-193-AC-9 alone; HY-04 and HY-07 cite FR-331-AC-29 and cover only its step and time-stop clauses. Add FR-331-AC-29 to HY-08's citation so every clause of AC-29 maps to a vector. | spec/test-cases/TC-373-hybrid-verdict-map.md:33; spec/objects/interfaces/FR-331-backend-provider-envelope.md:380 |

## Verdict

One low trace gap. ID allocation, AC-to-TC links and validation are otherwise clean; the FR-331 validation failures are pre-existing and identical on main, as the author reported.
